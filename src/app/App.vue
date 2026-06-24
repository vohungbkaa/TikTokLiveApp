<template>
  <div class="app-layout">
    <aside class="sidebar glass-panel-nav">
      <div class="logo">🚀 OfMe Live</div>
      <nav class="nav-menu" ref="menuRef">
        <router-link 
          v-for="item in menuItems" 
          :key="item.id" 
          :to="item.path" 
          class="nav-item" 
          active-class="active"
        >
          <span class="item-text">{{ item.name }}</span>
          <svg class="drag-handle" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="9" cy="12" r="1"/><circle cx="9" cy="5" r="1"/><circle cx="9" cy="19" r="1"/><circle cx="15" cy="12" r="1"/><circle cx="15" cy="5" r="1"/><circle cx="15" cy="19" r="1"/>
          </svg>
        </router-link>
      </nav>
      
    </aside>
    <main class="main-view">
      <!-- Standard Top Bar -->
      <header class="top-bar">
        <div class="user-menu-container" ref="menuContainer">
          <button class="avatar-btn" @click="toggleMenu" title="Tài khoản">
            <span class="icon">👤</span>
          </button>
          
          <div class="dropdown-menu" v-if="isMenuOpen">
            <div class="dropdown-item" @click="goToSettings">
              <span class="icon">⚙️</span> Cài đặt
            </div>
            <div class="dropdown-item text-danger" @click="handleLogout">
              <span class="icon">🚪</span> Đăng xuất
            </div>
          </div>
        </div>
      </header>

      <div class="content-area">
        <router-view />
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import { useLocalStorage } from '@vueuse/core';
import Sortable from 'sortablejs';

const router = useRouter();
const isMenuOpen = ref(false);
const menuContainer = ref<HTMLElement | null>(null);
const menuRef = ref<HTMLElement | null>(null);

const menuItems = useLocalStorage('app-menu-order', [
  { id: 'products', name: 'Kho Hàng', path: '/products' },
  { id: 'setup', name: 'Cấu Hình Live', path: '/live-setup' },
  { id: 'console', name: 'Màn Hình Live', path: '/live-console' },
  { id: 'orders', name: 'Đơn Hàng', path: '/order-review' }
]);

const toggleMenu = () => {
  isMenuOpen.value = !isMenuOpen.value;
};

const goToSettings = () => {
  isMenuOpen.value = false;
  router.push('/settings');
};

const handleLogout = () => {
  isMenuOpen.value = false;
  alert('Đăng xuất thành công!'); // Sẽ thay bằng logic thật sau
};

// Handle click outside
const handleClickOutside = (event: MouseEvent) => {
  if (menuContainer.value && !menuContainer.value.contains(event.target as Node)) {
    isMenuOpen.value = false;
  }
};

onMounted(() => {
  document.addEventListener('click', handleClickOutside);
  
  // Initialize drag and drop
  if (menuRef.value) {
    Sortable.create(menuRef.value, {
      animation: 150,
      ghostClass: 'sortable-ghost',
      handle: '.drag-handle',
      onEnd: (evt) => {
        const oldIndex = evt.oldIndex;
        const newIndex = evt.newIndex;
        if (oldIndex !== undefined && newIndex !== undefined) {
          const items = [...menuItems.value];
          const [movedItem] = items.splice(oldIndex, 1);
          items.splice(newIndex, 0, movedItem);
          menuItems.value = items;
        }
      }
    });
  }
});

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside);
});
</script>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
  background: var(--bg-color);
  color: var(--text-main);
}

.sidebar {
  width: 260px;
  display: flex;
  flex-direction: column;
  padding: 1.5rem;
  border-right: 1px solid var(--border);
}

.logo {
  font-size: 1.5rem;
  font-weight: 700;
  margin-bottom: 2rem;
  background: linear-gradient(135deg, #a78bfa, #60a5fa);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.nav-menu {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  flex: 1;
}

.nav-item {
  text-decoration: none;
  color: var(--text-muted);
  padding: 1rem 1.5rem;
  border-radius: 12px;
  font-weight: 500;
  transition: all 0.2s;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.item-text {
  flex: 1;
}

.drag-handle {
  opacity: 0;
  cursor: grab;
  color: var(--text-muted);
  transition: opacity 0.2s;
}

.drag-handle:active {
  cursor: grabbing;
}

.nav-item:hover .drag-handle {
  opacity: 0.5;
}

.drag-handle:hover {
  opacity: 1 !important;
  color: var(--primary);
}

.sortable-ghost {
  opacity: 0.4;
  background: rgba(255, 255, 255, 0.1);
  border: 1px dashed rgba(255, 255, 255, 0.3);
}

.nav-item:hover {
  background: rgba(255, 255, 255, 0.05);
  color: var(--text-main);
}

.nav-item.active {
  background: rgba(59, 130, 246, 0.15);
  color: var(--primary);
  font-weight: 600;
  border-left: 3px solid var(--primary);
}



.main-view {
  flex: 1;
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.top-bar {
  height: 72px;
  flex-shrink: 0;
  display: flex;
  justify-content: flex-end;
  align-items: center;
  padding: 0 2rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  background: transparent;
  position: relative;
  z-index: 1000;
}

.content-area {
  flex: 1;
  overflow: hidden;
  position: relative;
  display: flex;
  flex-direction: column;
}

.user-menu-container {
  position: relative;
}

.avatar-btn {
  width: 44px;
  height: 44px;
  border-radius: 50%;
  background: linear-gradient(135deg, rgba(167, 139, 250, 0.2), rgba(244, 114, 182, 0.2));
  border: 1px solid rgba(255, 255, 255, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s;
  color: var(--text-main);
  font-size: 1.2rem;
}

.avatar-btn:hover {
  background: linear-gradient(135deg, rgba(167, 139, 250, 0.3), rgba(244, 114, 182, 0.3));
  transform: scale(1.05);
}

.dropdown-menu {
  position: absolute;
  top: calc(100% + 0.5rem);
  right: 0;
  width: 200px;
  background: rgba(15, 23, 42, 0.9);
  backdrop-filter: blur(16px);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 0.5rem;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
  animation: slideDown 0.2s ease-out forwards;
  transform-origin: top right;
}

@keyframes slideDown {
  from { opacity: 0; transform: scale(0.95) translateY(-10px); }
  to { opacity: 1; transform: scale(1) translateY(0); }
}

.dropdown-item {
  display: flex;
  align-items: center;
  gap: 0.8rem;
  padding: 0.8rem 1rem;
  border-radius: 8px;
  cursor: pointer;
  color: var(--text-main);
  font-weight: 500;
  transition: all 0.2s;
}

.dropdown-item:hover {
  background: rgba(255, 255, 255, 0.1);
}

.dropdown-item .icon {
  font-size: 1.1rem;
}

.text-danger {
  color: #f87171 !important;
}

.text-danger:hover {
  background: rgba(239, 68, 68, 0.15) !important;
}


</style>