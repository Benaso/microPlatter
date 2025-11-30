<template>
  <el-container class="main-container">
    <el-header class="header-section">
      <el-container class="header-content">
        <el-aside width="400px" class="button-area">
          <el-button 
            class="start-button"
            @click="onStartRecording"
            :disabled="isRecording"
          >
            <span class="button-text">Start Recording</span>
            <img src="../assets/startlogo.png" class="button-logo">
          </el-button>

          <el-button
            class="stop-button"
            @click="onStopRecording"
            :disabled="!isRecording"
          >
            <span class="button-text">Stop Recording</span>
            <img src="../assets/stoplogo.png" class="button-logo">
          </el-button>

        </el-aside>
        
        <el-main class="chart-area" @mouseenter="showReminder" @mouseleave="startHideTimer">
          <div ref="chartContainer" class="chart-container"></div>
          
          <div :class="['reminder-box', {'is-visible': isReminderVisible}]" @mouseenter="cancelHideTimer" @mouseleave="startHideTimer">
            <div class="reminder-header">
              <span>Tip:</span>
              <button @click="closeReminder" class="close-button">×</button>
            </div>
            <div class="reminder-content">
              <ul>
                <li><span class="color-dot red-dot"></span>2~3 minutes</li>
                <li><span class="color-dot yellow-dot"></span>1~2 minutes</li>
                <li><span class="color-dot green-dot"></span>Under 1 minute</li>
              </ul>
            </div>
          </div>
        </el-main>
      </el-container>
    </el-header>

    <el-main class="table-section">
      <el-input 
        placeholder="Search by Name/Comment" 
        v-model="searchQuery" 
        class="table-search-input">
        <template #prefix>
          <el-icon><Search /></el-icon> 
        </template>
      </el-input>

      <div class="table-placeholder">
        <el-table
          :data="filteredTableData"
          style="width: 100%"
          :row-class-name="tableRowClassName">
          <el-table-column
            prop="date"
            label="DATE"
            align="center"
            width="180">
          </el-table-column>
          <el-table-column
            prop="name"
            label="NAME"
            align="center"
            width="180">
          </el-table-column>
          <el-table-column
            prop="comment"
            label="COMMENT"
            align="center"
            width="200">
          </el-table-column>
          <el-table-column
            prop="option"
            label="OPTION"
            align="center">
            <template v-slot:default="scope">
            <div style="display: flex; justify-content: center; align-items: center;"> 
              <el-button
                class="action-button"
                @click.stop="onPlay(scope.row)"
                :disabled="isRecording"
              >
                Play
              </el-button>
            </div>
          </template>
          </el-table-column>
        </el-table>
      </div>
    </el-main>
  </el-container>
</template>

<script>
import * as echarts from 'echarts'
import { invoke } from '@tauri-apps/api/core'
import { Search } from '@element-plus/icons-vue'

export default {
  name: 'RecordingDashboard',
  components: { Search },

  data() {
    return {
      isRecording: false,    
      isReminderVisible: false,
      reminderTimeout: null,
  searchQuery: '',
  tableData: [],
  sessionCount: 0,
  // time cost ranges used in the reminder box; each item maps to a color class
      timeCostRanges: [
        { label: '2~3 minutes', colorClass: 'red-dot' },
        { label: '1~2 minutes', colorClass: 'yellow-dot' },
        { label: 'Under 1 minute', colorClass: 'green-dot' }
      ]
    }
  },

  computed: {
    filteredTableData() {
      if (!this.searchQuery) return this.tableData
      const query = this.searchQuery.toLowerCase()
      return this.tableData.filter(item =>
        item.name.toLowerCase().includes(query) ||
        item.comment.toLowerCase().includes(query)
      )
    }
  },

  mounted() {
  this.initChart()
  this.loadSessions()
  },

  beforeUnmount() {
    clearTimeout(this.reminderTimeout)
    if (this._myChart) {
      window.removeEventListener('resize', () => this._myChart.resize())
      this._myChart.dispose()
      this._myChart = null
    }
  },

  methods: {
    /* -----------------------------------------
       核心：调用 Tauri 后端 Rust API
    ------------------------------------------*/

    async onStartRecording() {
      try {
        this.isRecording = true
        const sessionName = "New Session"
        const sessionId = await invoke("start_recording", {
          sessionName,
          description: "Started from UI"
        })
        console.log("Start recording => sessionId =", sessionId)
      } catch (e) {
        console.error("Start recording error:", e)
        this.isRecording = false
      }
    },

    async onStopRecording() {
      try {
        const result = await invoke("stop_recording")
        console.log("Stop recording =>", result)
  // reload sessions so the table reflects the newly stopped session
  this.loadSessions()
      } catch (e) {
        console.error("Stop recording error:", e)
      } finally {
        this.isRecording = false
      }
    },

    async onPlay(row) {
      try {
        const sessionId = row.sessionId || row.id || null
        if (!sessionId) {
          console.warn('No session id for row', row)
          return
        }
        console.log('Invoking play_recording for session', sessionId)
        const result = await invoke('play_recording', { sessionId })
        console.log('Playback result:', result)
      } catch (e) {
        console.error('Play recording error:', e)
      }
    },

    async loadSessions() {
      try {
        const sessions = await invoke('list_sessions')
        // sessions is expected to be an array of { id, name, description, created_at, event_count }
        this.tableData = (sessions || []).map(s => ({
          sessionId: s.id,
          date: s.created_at ? s.created_at.split('T')[0] : '',
          name: s.name || '',
          comment: s.description || '',
          time_cost: s.time_cost || 0,
          option: '操作'
        }))

        // update session count and refresh gauge + color distribution
        this.sessionCount = this.tableData.length

        // compute distribution counts
        const greenCount = this.tableData.filter(r => r.time_cost > 0 && r.time_cost < 60).length
        const yellowCount = this.tableData.filter(r => r.time_cost >= 60 && r.time_cost < 120).length
        const redCount = this.tableData.filter(r => r.time_cost >= 120 && r.time_cost < 180).length
        const total = greenCount + yellowCount + redCount

        if (this._myChart) {
          try {
            if (total > 0) {
              const barData = [greenCount, yellowCount, redCount]
              this._myChart.setOption({ series: [{ data: barData, type: 'bar', itemStyle: { color: function(params) { return ['#43A047','#FFD54F','#EF5350'][params.dataIndex] } } }] })
            } else {
              // fallback: no sessions -> neutral single slice
              this._myChart.setOption({ series: [{ data: [0,0,0], type: 'bar' }] })
            }
          } catch (e) {
            console.warn('Failed to update gauge:', e)
          }
        }
      } catch (e) {
        console.error('Failed to load sessions:', e)
      }
    },

    /* -----------------------------------------
       表格样式、搜索、小提示
    ------------------------------------------*/
    tableRowClassName({ row }) {
      // compute class by time_cost in seconds
      if (!row || typeof row.time_cost === 'undefined') return ''
      const cls = this.timeCostClass(row.time_cost)
      return cls
    },

    timeCostClass(time_cost) {
      if (time_cost >= 120 && time_cost < 180) return 'time-cost-red'
      if (time_cost >= 60 && time_cost < 120) return 'time-cost-yellow'
      if (time_cost > 0 && time_cost < 60) return 'time-cost-green'
      return ''
    },

    showReminder() {
      this.cancelHideTimer()
      this.isReminderVisible = true
      this.startHideTimer()
    },

    // start a 3s hide timer (used on mouseleave)
    startHideTimer(delay = 2000) {
      this.cancelHideTimer()
      this.reminderTimeout = setTimeout(() => {
        this.isReminderVisible = false
        this.reminderTimeout = null
      }, delay)
    },

    // cancel any pending hide timer (used on mouseenter)
    cancelHideTimer() {
      if (this.reminderTimeout) {
        clearTimeout(this.reminderTimeout)
        this.reminderTimeout = null
      }
    },

    hideReminder() {
      // immediate hide (used by close button)
      this.cancelHideTimer()
      this.isReminderVisible = false
    },

    closeReminder() {
      this.hideReminder()
    },

    initChart() {
      const myChart = echarts.init(this.$refs.chartContainer)
      const option = {
        grid: { left: '6%', right: '6%', top: '6%', bottom: '2%', containLabel: true },
        xAxis: { type: 'category', data: ['Green','Yellow','Red'], axisTick: { show: false }, axisLine: { show: false }, axisLabel: { color: '#6b7280' } },
        yAxis: { type: 'value', axisTick: { show: false }, axisLine: { show: false }, splitLine: { show: false }, axisLabel: { show: false } },
        series: [ { data: [0,0,0], type: 'bar', itemStyle: { borderRadius: 6 }, emphasis: { focus: 'series' } } ],
        tooltip: { trigger: 'axis' }
      }

      myChart.setOption(option)
      window.addEventListener('resize', () => myChart.resize())
      this._myChart = myChart
    }
  }
}
</script>

<style scoped>
/* Prevent horizontal overflow for this page/component */
.main-container {
  overflow-x: hidden; /* stop page-level horizontal scroll */
}

/* ensure inner boxes include padding in width calculations */
.main-container,
.table-section,
.table-placeholder,
.chart-container,
.el-table {
  box-sizing: border-box;
  max-width: 100%;
}

/* Ensure table internals can shrink below content width when needed */
.table-placeholder :deep(.el-table__inner-wrapper) {
  min-width: 0;
}

/* ======================== 全局布局样式：确保无全局滚动条 ======================== */
.main-container {
  display: flex;
  flex-direction: column;
  /* use 100% so component fills parent (which is already set to 100%)
     using 100vh inside a percent-height parent can create double-height
     and cause page-level scrollbars */
  height: 100%;
  padding: 0;
  margin: 0;
  box-sizing: border-box;
  /* do not force overflow here - allow the table-section to handle scrolling */
}

.header-section {
  height: 250px; 
  padding: 5px;
  background-color: white !important; 
  flex-shrink: 0; 
}

.header-content {
  height: 100%;
}

/* ======================== C 区: 按钮区域 (恢复原始水平布局) ======================== */
.button-area {
  background-color: transparent !important; 
  display: flex;
  flex-direction: row; /* 确保按钮水平并排 */
  align-items: center; 
  justify-content: flex-start;
  padding-right: 10px;
}

/* 按钮本体样式 - 恢复原始 100px 高度 */
.start-button, .stop-button {
  width: 150px; 
  height: 100px; /* 恢复为原始高度 */
  margin-right: 10px; /* 按钮之间的水平间距 */
  margin-bottom: 0; /* 移除不必要的垂直间距 */
  
  font-size: 16px;
  color: #192038; 
  
  display: flex; 
  flex-direction: column; 
  align-items: flex-start; 
  justify-content: space-around; 
  
  background-color: white !important;
  border: 1px solid #192038;
  border-radius: 4px;
  padding: 10px; 
  font-weight: bold; 
}

.start-button :deep(span), .stop-button :deep(span) {
  display: contents; 
  white-space: normal;
}

.button-text {
  align-self: flex-start; 
  white-space: nowrap;
}

.start-button:hover, 
.stop-button:hover {
  color: #192038 !important; 
  background-color: #f7f7f7 !important; 
  border-color: #c0c4cc !important; 
}

.button-logo {
  width: 35px; 
  height: 35px; 
  display: block; 
  align-self: flex-start; 
  margin-top: 5px; 
  margin-right: 0; 
  object-fit: contain;
}

/* Action button used inside table rows (matches main buttons but smaller) */
.action-button {
  height: 36px;
  min-width: 72px;
  padding: 6px 12px;
  margin: 0 6px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  background-color: white !important;
  border: 1px solid #192038;
  border-radius: 4px;
  font-weight: bold;
  font-size: 14px;
}


/* ======================== D 区: 图表区域 (不变) ======================== */
.chart-area {
  background-color: transparent !important;
  padding: 0 !important;
  display: flex;
  justify-content: center; 
  align-items: center;
  position: relative; 
  overflow: visible; 
  flex-grow: 1; 
}

.chart-container { 
  width: 100%; 
  height: 200px; 
}

/* ======================== Reminder Box 样式 (不变) ======================== */
.reminder-box {
  position: absolute;
  top: 8px;
  right: 0;

  width: 180px;
  background-color: white;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  padding: 8px 10px;
  box-shadow: 0 6px 10px rgba(0, 0, 0, 0.08);
  z-index: 100;

  transition: transform 0.3s ease-out;
  transform: translateX(110%);
}
.reminder-box.is-visible { transform: translateX(0); }
.reminder-header {
  display: flex;
  align-items: center;
  font-weight: 600;
  font-size: 13px;
  color: #111827;
  margin-bottom: 6px;
}
.close-button {
  background: none;
  border: none;
  font-size: 14px;
  cursor: pointer;
  color: #6b7280;
  margin-left: auto;
}
.reminder-content { padding-left: 0; }
.reminder-content ul {
  list-style: none;
  padding: 0;
  margin: 0;
}
.reminder-content li {
  display: flex;
  align-items: center;
  margin-bottom: 4px;
  font-size: 12px;
  color: #374151;
}
.color-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  margin-right: 6px;
  flex-shrink: 0;
}
.red-dot { background-color: #EF5350; }
.yellow-dot { background-color: #FFD54F; }
.green-dot { background-color: #43A047; }


/* ======================== B 区: Table (确保局部滚动和搜索框集成) ======================== */
.table-section {
  padding: 10px 20px 20px 20px;
  background-color: #f7f7f7;
  flex-grow: 1;
  overflow-y: auto;
  overflow-x: hidden;

  display: flex;
  flex-direction: column;
  /* allow this flex item to shrink properly inside its parent so internal
     overflow works as expected */
  min-height: 0;
}

.table-search-input {
    width: 300px; 
    margin-bottom: 10px; 
    align-self: flex-start; 
    margin-left: 0; 
}

.table-placeholder {
  width: 100%;
  flex-grow: 1;
  overflow: hidden;
  /* allow nested table to size/scroll correctly */
  min-height: 0;
}

/* 覆盖 Element UI 内部样式，确保表格本身可以适应高度 */
.el-table {
    flex-grow: 1;
    height: 100%;
}
.el-table :deep(.el-table__inner-wrapper) {
    height: 100%;
}
.el-table :deep(.el-table__body-wrapper) {
    overflow-y: auto;
}


/* 表格行样式 (不变) */
.el-table :deep(.el-table__cell) {
  padding: 20px 0; 
  font-size: 14px; 
}
.el-table :deep(.warning-row) { background: oldlace !important; }
.el-table :deep(.success-row) { background: #f0f9eb !important; }
/* Time cost based row highlighting */
.el-table :deep(.time-cost-red) { background: rgba(229,57,53,0.08) !important; }
.el-table :deep(.time-cost-yellow) { background: rgba(253,216,53,0.08) !important; }
.el-table :deep(.time-cost-green) { background: rgba(67,160,71,0.12) !important; }
</style>