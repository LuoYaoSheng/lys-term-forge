import { defineConfig } from 'vitepress';

export default defineConfig({
  title: 'TermForge',
  description: '跨平台 SSH / SFTP / Runbook 运维工作台',
  lang: 'zh-CN',
  base: '/',
  cleanUrls: true,
  ignoreDeadLinks: true,

  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/favicon.svg' }],
    ['meta', { name: 'author', content: 'LuoYaoSheng' }],
    ['meta', { name: 'keywords', content: 'SSH,SFTP,Runbook,运维工作台,终端,桌面工具,Tauri,TermForge' }],
    ['meta', { property: 'og:type',        content: 'website' }],
    ['meta', { property: 'og:site_name',   content: 'TermForge' }],
    ['meta', { property: 'og:title',       content: 'TermForge — 跨平台运维工作台' }],
    ['meta', { property: 'og:description', content: 'SSH / SFTP / Runbook 一体化运维工具，跨平台桌面应用。' }],
    ['meta', { property: 'og:url',         content: 'https://term.open.i2kai.com/' }],
    ['meta', { property: 'og:locale',      content: 'zh_CN' }],
    ['meta', { name: 'twitter:card',        content: 'summary_large_image' }],
    ['meta', { name: 'twitter:title',       content: 'TermForge — 跨平台运维工作台' }],
    ['meta', { name: 'twitter:description', content: 'SSH / SFTP / Runbook 一体化运维工具。' }],
    ['meta', { name: 'theme-color', content: '#646cff' }],
  ],

  themeConfig: {
    nav: [
      { text: '首页', link: '/' },
    ],
    sidebar: [],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/LuoYaoSheng/lys-term-forge' },
      { icon: 'github', link: 'https://gitee.com/luoyaosheng/lys-term-forge', ariaLabel: 'Gitee' },
    ],
  },
});
