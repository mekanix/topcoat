;(function () {
  try {
    var mode = localStorage.getItem('topcoat-theme-mode')
    if (mode !== 'light' && mode !== 'dark') {
      mode = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
    }
    document.documentElement.setAttribute('data-theme', mode)
  } catch (e) {
    // Ignore storage and media query errors and keep the light theme.
  }
})()
