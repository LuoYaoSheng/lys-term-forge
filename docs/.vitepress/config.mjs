import { defineConfig } from 'vitepress';

export default defineConfig({
  title: 'TermForge',
  description: '跨平台 SSH / SFTP / Runbook 运维工作台',
  lang: 'zh-CN',
  cleanUrls: true,
  ignoreDeadLinks: true,
  themeConfig: {
    nav: [
      { text: '首页', link: '/' },
    ],
    sidebar: [],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/LuoYaoSheng/lys-term-forge' },
    ],
  },
});
