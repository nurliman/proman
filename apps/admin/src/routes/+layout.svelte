<script>
  import "../app.css";
</script>

<div class="flex min-h-screen bg-white">
  <nav class="bg-blue-700">
    <a href="/">Home</a>
    <a href="/about">About</a>
    <a href="/settings">Settings</a>
  </nav>

  <main>
    <slot />
  </main>
</div>
