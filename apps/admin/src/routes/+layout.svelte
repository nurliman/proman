<script lang="ts">
  import P from "flowbite-svelte/P.svelte";
  import ThemeSwitcher from "$lib/components/ThemeSwitcher.svelte";
  import "@fontsource/quicksand";
  import "../app.css";

  const menu = [
    { name: "Projects", href: "#projects" },
    { name: "Deployments", href: "#deployments" },
    { name: "Secrets", href: "#secrets" },
    { name: "Users", href: "#users" },
    { name: "Settings", href: "#settings" },
  ];
</script>

<div class="flex min-h-screen bg-white dark:bg-gray-900">
  <aside
    class="bg-base-200 flex w-80 flex-col border-r border-dashed border-gray-200 dark:border-gray-700"
  >
    <div class="flex h-20 items-center justify-center">Insert logo</div>
    <div class="flex flex-1 flex-col px-8">
      {#each menu as { name, href }}
        <a {href} class="mb-2 flex items-center space-x-2 p-4">
          <P
            space="wide"
            weight="semibold"
            class="!text-gray-500 hover:!text-gray-900 dark:hover:!text-white"
            >{name}</P
          >
        </a>
      {/each}
    </div>
    <div class="py-4 px-8">
      <ThemeSwitcher />
    </div>
  </aside>

  <main>
    <slot />
  </main>
</div>
