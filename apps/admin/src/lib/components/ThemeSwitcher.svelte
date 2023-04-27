<script lang="ts">
  import { onMount } from "svelte";
  import { Icon, Moon, Sun, type IconSource } from "svelte-hero-icons";
  import Button from "flowbite-svelte/Button.svelte";
  import ChevronUp from "flowbite-svelte/ChevronUp.svelte";
  import Dropdown from "flowbite-svelte/Dropdown.svelte";
  import DropdownItem from "flowbite-svelte/DropdownItem.svelte";

  type Theme = {
    name: "light" | "dark";
    icon: IconSource;
  };

  const themes: Readonly<Theme[]> = [
    { name: "light", icon: Sun },
    { name: "dark", icon: Moon },
  ];

  let selectedTheme: Theme = themes[1];

  const titleCase = (str: string) => {
    return str[0].toUpperCase() + str.slice(1);
  };

  const changeTheme = (theme: Theme) => {
    localStorage.setItem("color-theme", theme.name);
    window.document.documentElement.classList.toggle("dark", theme.name === "dark");
    selectedTheme = theme;
  };

  onMount(() => {
    if (localStorage.getItem("color-theme") === "light") {
      window.document.documentElement.classList.remove("dark");
      selectedTheme = themes[0];
    }
  });
</script>

<Button size="xs" color="alternative">
  <Icon src={selectedTheme.icon} size="12" class="mr-2" />
  {titleCase(selectedTheme.name)}
  <ChevronUp size="12" class="ml-2" />
</Button>
<Dropdown placement="top-start">
  {#each themes as theme}
    <DropdownItem on:click={() => changeTheme(theme)}>
      {titleCase(theme.name)}
    </DropdownItem>
  {/each}
</Dropdown>
