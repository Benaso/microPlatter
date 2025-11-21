<template>
  <el-container class="main-container">
    <el-header class="header-section">
      <el-container class="header-content">
        <el-aside width="400px" class="button-area">
          
          <el-button class="start-button">  
            <span class="button-text">Start Recording</span>
            <img src="../assets/startlogo.png" alt="startlogo logo" class="button-logo">
          </el-button>
          
          <el-button class="stop-button">
            <span class="button-text">Stop Recording</span>
            <img src="../assets/stoplogo.png" alt="stoplogo logo" class="button-logo">
          </el-button>
          
        </el-aside>
        
        <el-main class="chart-area" @mouseenter="showReminder" @mouseleave="hideReminder">
          <div ref="chartContainer" class="chart-container"></div>
          
          <div :class="['reminder-box', {'is-visible': isReminderVisible}]">
            <div class="reminder-header">
              <span>Reminder:</span>
              <button @click="closeReminder" class="close-button">×</button>
            </div>
            <div class="reminder-content">
              <ul>
                <li><span class="color-dot red-dot"></span>Over 5 minutes</li>
                <li><span class="color-dot yellow-dot"></span>1–5 minutes</li>
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
            width="180">
          </el-table-column>
          <el-table-column
            prop="name"
            label="NAME"
            width="180">
          </el-table-column>
          <el-table-column
            prop="comment"
            label="COMMENT"
            width="200">
          </el-table-column>
          <el-table-column
            prop="option"
            label="OPTION">
          </el-table-column>
        </el-table>
      </div>
    </el-main>
  </el-container>
</template>

<script>
import * as echarts from 'echarts'; 
import { Search } from '@element-plus/icons-vue' 

export default { 
  name: 'RecordingDashboard',
  components: {
    Search 
  },
  data() {
    return {
      isReminderVisible: false, 
      reminderTimeout: null,    
      searchQuery: '', // 搜索内容
      tableData: [{
        date: '2016-05-02',
        name: '王小虎',
        comment: '一些评论',
        option: '操作'
      }, {
        date: '2016-05-04',
        name: '王小虎',
        comment: '一些评论',
        option: '操作'
      }, {
        date: '2016-05-01',
        name: '张三丰',
        comment: '记录开始',
        option: '操作'
      }, {
        date: '2016-05-03',
        name: '李四',
        comment: '一些评论',
        option: '操作'
      }, {
        date: '2016-05-03',
        name: '王小虎',
        comment: '一些评论',
        option: '操作'
      }, {
        date: '2016-05-03',
        name: '王小虎',
        comment: '一些评论',
        option: '操作'
      }, {
        date: '2016-05-03',
        name: '王小虎',
        comment: '一些评论',
        option: '操作'
      }, {
        date: '2016-05-03',
        name: '王小虎',
        comment: '一些评论',
        option: '操作'
      }, {
        date: '2016-05-03',
        name: '王小虎',
        comment: '一些评论',
        option: '操作'
      }, {
        date: '2016-05-03',
        name: '王小虎',
        comment: '一些评论',
        option: '操作'
      }]
    };
  },
  computed: {
    filteredTableData() {
      if (!this.searchQuery) {
        return this.tableData;
      }
      const query = this.searchQuery.toLowerCase();
      return this.tableData.filter(item => {
        return item.name.toLowerCase().includes(query) ||
               item.comment.toLowerCase().includes(query);
      });
    }
  },
  mounted() {
    this.initChart();
  },
  beforeUnmount() {
    clearTimeout(this.reminderTimeout); 
    if (this._myChart) {
      window.removeEventListener('resize', () => this._myChart.resize());
      this._myChart.dispose();
      this._myChart = null;
    }
  },
  methods: {
    tableRowClassName({row, rowIndex}) {
      if (rowIndex % 3 === 1) { 
        return 'warning-row';
      } else if (rowIndex % 3 === 2) { 
        return 'success-row';
      }
      return '';
    },
    showReminder() {
      clearTimeout(this.reminderTimeout);
      this.isReminderVisible = true;
      this.reminderTimeout = setTimeout(() => {
        this.isReminderVisible = false;
      }, 10000); 
    },
    hideReminder() {
      clearTimeout(this.reminderTimeout);
      this.isReminderVisible = false;
    },
    closeReminder() {
      clearTimeout(this.reminderTimeout);
      this.isReminderVisible = false;
    },
    initChart() {
      const myChart = echarts.init(this.$refs.chartContainer);
      const option = {
        series: [
          {
            type: 'gauge',
            startAngle: 180,
            endAngle: 0,
            center: ['50%', '65%'], 
            radius: '100%', 
            min: 0,
            max: 200, 
            splitNumber: 4, 
            pointer: { show: false },
            progress: {
              show: true,
              overlap: false,
              roundCap: true, 
              width: 15, 
              itemStyle: { color: '#FFA726' },
            },
            axisLine: {
              lineStyle: {
                width: 15, 
                color: [ 
                  [0.33, '#EF5350'], 
                  [0.66, '#FFD54F'], 
                  [1, '#A5D6A7']      
                ],
                opacity: 0.3 
              }
            },
            axisTick: { show: false },
            splitLine: { show: false },
            axisLabel: { show: false },
            detail: {
              fontSize: 40, 
              offsetCenter: [0, '-25%'], 
              valueAnimation: true,
              formatter: function (value) { return value + ''; },
              color: '#192038' 
            },
            title: {
              offsetCenter: [0, '20%'], 
              fontSize: 16,
              color: '#666', 
              show: true 
            },
            data: [
              {
                value: 130, 
                name: 'Recording Distribution' 
              }
            ]
          }
        ]
      };
      
      myChart.setOption(option);
      window.addEventListener('resize', () => myChart.resize());
      this._myChart = myChart;
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

/* Prevent images from forcing horizontal overflow */
.main-container :deep(img) {
  max-width: 100%;
  height: auto;
  display: block;
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
  top: 10px;
  right: 0; 
  
  width: 250px;
  background-color: white; 
  border: 1px solid #333; 
  border-radius: 8px;
  padding: 15px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 100;

  transition: transform 0.5s ease-out; 
  transform: translateX(105%); 
}
.reminder-box.is-visible { transform: translateX(0); }
.reminder-header {
  display: flex;
  align-items: center;
  font-weight: bold;
  font-size: 16px;
  color: #192038;
  margin-bottom: 10px;
}
.close-button {
  background: none;
  border: none;
  font-size: 18px;
  cursor: pointer;
  color: #666;
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
    margin-bottom: 5px;
    font-size: 14px;
    color: #333;
}
.color-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%; 
    margin-right: 8px; 
    flex-shrink: 0; 
}
.red-dot { background-color: #EF5350; }
.yellow-dot { background-color: #FFD54F; }
.green-dot { background-color: #A5D6A7; }


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
</style>