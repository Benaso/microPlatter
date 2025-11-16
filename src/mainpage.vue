<template>
  <el-container style="height: 100%; box-sizing: border-box; border: 1px solid #eee">
  <el-aside width="220px" class="dark-sidebar">
      <div class="sidebar-header">
        <img src="./assets/logo.png" alt="MicroPlatter Logo" class="sidebar-logo">
        MicroPlatter
      </div>

      <el-menu 
        :default-active="$route.path" 
        @select="onMenuSelect"
        class="sidebar-menu"
        :collapse="false" 
      >
      <div class="menu-section-title">TOOLBOX</div>
        
      <el-menu-item index="/Recording">
          <i class="el-icon-video-play"></i>
          <span slot="title">
            <img src="./assets/recordinglogo.png" alt="recording logo" class="sub-logo">
            Recording
          </span>
      </el-menu-item>
      <el-menu-item index="/Configuration">
          <i class="el-icon-setting"></i>
          <span slot="title">
            <img src="./assets/configureLogo.png" alt="configuration logo" class="sub-logo">
            Configuration
          </span>
      </el-menu-item>
      <div class="menu-section-title settings-spacer">SETTINGS</div>
        
      <el-menu-item index="/Theme">
          <i class="el-icon-finished"></i>
          <span slot="title">
            <img src="./assets/theme.png" alt="theme logo" class="sub-logo">
            Theme
          </span>
      </el-menu-item>
      <el-menu-item index="/Examples">
        <i class="el-icon-copy-document"></i>
        <span slot="title">
          <img src="./assets/example.png" alt="example logo" class="sub-logo">
          Examples
        </span>
      </el-menu-item>
      </el-menu>
    </el-aside>

      <el-container style="height: 100%; min-height: 100%; box-sizing: border-box;">
        <el-header style="display:flex; justify-content:space-between; align-items:center; padding: 0 16px;">
          <div style="font-weight:600">{{ currentTitle }}</div>
        
        </el-header>
  <el-main style="padding: 16px;">
          <router-view />
        </el-main>
    </el-container>
  </el-container>
</template>

<script>
export default {
  name: 'MainPage',
  data() {
    return { currentTitle: this.$route.meta && this.$route.meta.title ? this.$route.meta.title : '主标题' }
  },
  methods: {
    onMenuSelect(index) {
      if (!index) return
      // menu item index is the route path
      const path = typeof index === 'string' ? index : index && index.index
      console.log('derived path', path)
      if (!path) return
      // use direct push(path) for clarity
      this.$router.push(path).then(() => {
        console.log('navigated to', path)
      }).catch(err => {
        console.error('navigation error', err)
      })
    }
  }
  ,
  watch: {
    '$route'(to) {
      const t = to.meta && to.meta.title ? to.meta.title : '主标题'
      this.currentTitle = t
      document.title = t
    }
  },
  mounted() {
    // ensure document.title matches on first load
    const t = this.$route.meta && this.$route.meta.title ? this.$route.meta.title : '主标题'
    document.title = t
  }
}
</script>

<style scoped>
/* 确保 el-header 样式被正确引用，尽管在这里不直接影响侧边栏，但保留 */
.el-header { 
    background: #f5f7fa; 
    height: 56px; 
}

/* 1. 侧边栏整体样式 (深色背景 & 字体栈) */
.dark-sidebar {
    background-color: #192038; /* 深蓝灰色 */
    color: #ffffff; /* 默认文本颜色 */
    /* 引入现代系统字体栈，匹配截图中的简洁无衬线字体风格 */
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji';
}

/* 2. Logo/头部样式 */
.sidebar-header {
    height: 60px;
    display: flex;
    align-items: center;
    padding-left: 20px;
    margin-bottom: 10px;
    color: #ffffff;
    font-size: 18px; /* Logo 文本大小 */
    font-weight: 500; /* Logo 文本字重 */
}
/* Logo 图片样式 (替换了原占位符样式) */
.sidebar-header .sidebar-logo {
    width: 30px; /* 图片宽度 */
    height: 30px; /* 图片高度 */
    margin-right: 10px;
    object-fit: contain;
}

.sub-logo {
    width: 20px; /* 图片宽度 */
  height: 20px; /* 图片高度 */
  display: inline-block;
  vertical-align: middle; /* 与文本垂直居中 */
  margin-right: 8px;
  object-fit: contain;
}

/* 3. 菜单样式重置 */
.sidebar-menu {
    border-right: none; /* 移除默认右侧边框 */
    background-color: #192038 !important; 
}

/* 4. 分组标题样式 (TOOLBOX, SETTINGS) */
.menu-section-title {
    color: #b6c1d7; /* 浅色文本 */
    font-size: 12px; /* 较小的字体大小 */
    font-weight: 600; /* 半粗体，突出分组标题 */
    padding: 20px 0 10px 30px; /* 调整间距 */
    text-transform: uppercase; /* 转换为大写 */
}
.settings-spacer {
    margin-top: 20px; /* SETTINGS 上方留白 */
}

/* 5. 菜单项默认样式 */
.sidebar-menu .el-menu-item {
    color: #b6c1d7 !important; /* 浅色文本 */
    height: 48px;
    line-height: 48px;
    padding-left: 30px !important; /* 调整对齐 */
    font-size: 14px; /* 明确菜单项字体大小 */
    font-weight: 400; /* 常规字重 */
}
.sidebar-menu .el-menu-item i {
    color: #b6c1d7; /* 默认图标颜色 */
}
/* 鼠标悬停时的样式 */
.sidebar-menu .el-menu-item:hover {
    background-color: #242c44 !important; 
    color: #ffffff !important;
}

/* 6. 选中 (Active) 菜单项样式 - 实现截图中的效果 */
.sidebar-menu .el-menu-item.is-active {
    background-color: #ffffff !important; /* 白色背景 */
    color: #192038 !important; /* 深色文本 */
    font-weight: 600; /* 半粗体，突出选中项 */
    border-left: 5px solid #409EFF; /* 左侧蓝色高亮条 */
    margin-left: -5px; /* 向左偏移，以便显示边框 */
}
.sidebar-menu .el-menu-item.is-active i {
    color: #192038 !important; /* 选中时图标变深 */
}
</style>