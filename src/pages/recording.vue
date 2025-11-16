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
        
        <el-main class="chart-area">
          <div ref="chartContainer" class="chart-container"></div>
        </el-main>
      </el-container>
    </el-header>

    <el-main class="table-section">
      <div class="table-placeholder">
        </div>
    </el-main>
  </el-container>
</template>

<script>
import * as echarts from 'echarts'; 

export default { 
  name: 'RecordingDashboard',
  mounted() {
    this.initChart();
  },
  beforeUnmount() {
    if (this._myChart) {
      window.removeEventListener('resize', () => this._myChart.resize());
      this._myChart.dispose();
      this._myChart = null;
    }
  },
  methods: {
    initChart() {
      // 初始化 ECharts 实例
      const myChart = echarts.init(this.$refs.chartContainer);
      
      const option = {
        series: [
          {
            type: 'gauge',
            // 半圆弧度
            startAngle: 180,
            endAngle: 0,
            // 调整中心点，使其在容器中居中显示
            center: ['50%', '65%'], 
            // 调整半径，使其填充容器
            radius: '100%', 
            min: 0,
            max: 200, 
            splitNumber: 4, 
            
            // 不显示指针
            pointer: {
              show: false
            },
            
            // 进度条（橙色指示块）
            progress: {
              show: true,
              overlap: false,
              roundCap: true, 
              width: 15, 
              itemStyle: {
                color: '#FFA726' 
              },
            },

            // 仪表盘背景线（轴线）样式
            axisLine: {
              lineStyle: {
                width: 15, 
                color: [ 
                  [0.25, '#EF5350'], 
                  [0.5, '#FFD54F'], 
                  [0.75, '#FFA726'], 
                  [1, '#66BB6A']    
                ],
                opacity: 0.3 
              }
            },
            
            axisTick: { show: false },
            splitLine: { show: false },
            axisLabel: { show: false },
            
            // 中心大数值 (130)
            detail: {
              fontSize: 40, 
              offsetCenter: [0, '-25%'], // 向上调整位置，使其位于半圆中心
              valueAnimation: true,
              formatter: function (value) {
                return value + ''; 
              },
              color: '#192038' 
            },
            
            // 底部文字 (Recording Distribution)
            title: {
              offsetCenter: [0, '20%'], // 向下调整位置
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

  // save reference for cleanup
  this._myChart = myChart;
    }
  }
}
</script>

<style scoped>
/* ======================== 基础布局样式 ======================== */
.main-container {
  flex-direction: column; 
  height: 100%; 
}

.header-section {
  height: 250px; 
  padding: 5px;
  background-color: white !important; 
}

.header-content {
  height: 100%;
}

/* ======================== C 区: 按钮区域 ======================== */
.button-area {
  background-color: transparent !important; 
  display: flex;
  flex-direction: row; 
  align-items: center; 
  justify-content: flex-start;
  padding-right: 10px;
}

/* 按钮本体样式 */
.start-button, .stop-button {
  width: 150px; 
  height: 100px; 
  margin-right: 10px; 
  margin-bottom: 0; 
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

/* 确保 Element UI 按钮内部的 span 不干扰 Flex 布局 */
.start-button :deep(span), .stop-button :deep(span) {
  display: contents; 
  white-space: normal;
}

/* 按钮内部的文本 */
.button-text {
  align-self: flex-start; 
  white-space: nowrap;
}


/* 鼠标悬停时的样式 */
.start-button:hover, 
.stop-button:hover {
  color: #192038 !important; 
  background-color: #f7f7f7 !important; 
  border-color: #c0c4cc !important; 
}

/* 图标样式 */
.button-logo {
  width: 35px; 
  height: 35px; 
  
  display: block; 
  align-self: flex-start; 
  
  margin-top: 5px; 
  margin-right: 0; 
  object-fit: contain;
}

/* ======================== D 区: 图表区域 (修改以填充) ======================== */
.chart-area {
  background-color: transparent !important;
  padding: 0 !important;
  display: flex;
  justify-content: center; /* 确保图表容器居中 */
  align-items: center;
}

.chart-container { /* 新增的 ECharts 容器样式 */
  width: 100%; 
  height: 200px; /* 固定高度，确保仪表盘可见 */
}

/* 移除原有的 chart-placeholder 样式，因为我们使用 chart-container */
/* .chart-placeholder {
  width: 250px; 
  height: 100%;
} */


/* ======================== B 区: Table ======================== */
.table-section {
  padding: 0 20px 20px 20px; 
  background-color: #f7f7f7; 
  overflow: auto;
}

.table-placeholder {
  width: 100%;
  height: 100%;
}
</style>