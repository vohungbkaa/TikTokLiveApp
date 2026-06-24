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
          <svg @click.prevent class="drag-handle" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
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
            <User :size="20" />
          </button>
          
          <div class="dropdown-menu" v-if="isMenuOpen">
            <div class="dropdown-header">
              <div class="user-info">
                <span class="user-name">Admin</span>
                <span class="user-role">Shop Manager</span>
              </div>
            </div>
            <div class="dropdown-divider"></div>
            <div class="dropdown-item" @click="goToSettings">
              <Settings class="icon" :size="18" /> Cài đặt
            </div>
            <div class="dropdown-divider"></div>
            <div class="dropdown-item text-danger" @click="handleLogout">
              <LogOut class="icon" :size="18" /> Đăng xuất
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
import { Settings, LogOut, User } from 'lucide-vue-next';

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
  
  if (menuRef.value) {
    Sortable.create(menuRef.value, {
      animation: 150,
      ghostClass: 'sortable-ghost',
      handle: '.drag-handle',
      forceFallback: true,
      fallbackOnBody: true,
      onEnd: (evt) => {
        const oldIndex = evt.oldIndex;
        const newIndex = evt.newIndex;
        if (oldIndex !== undefined && newIndex !== undefined && oldIndex !== newIndex) {
          // Revert DOM mutation so Vue doesn't throw patching errors
          const itemEl = evt.item;
          const parentEl = evt.from;
          const sibling = parentEl.children[oldIndex < newIndex ? oldIndex : oldIndex + 1];
          parentEl.insertBefore(itemEl, sibling || null);

          // Update Vue state
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
  width: 220px;
  background: rgba(15, 23, 42, 0.95);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 16px;
  padding: 0.5rem;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.4), 0 0 0 1px rgba(255, 255, 255, 0.05) inset;
  animation: slideDown 0.2s cubic-bezier(0.16, 1, 0.3, 1) forwards;
  transform-origin: top right;
  z-index: 9999;
}

@keyframes slideDown {
  from { opacity: 0; transform: scale(0.95) translateY(-10px); }
  to { opacity: 1; transform: scale(1) translateY(0); }
}

.dropdown-header {
  padding: 0.8rem 1rem;
}

.user-info {
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
}

.user-name {
  font-weight: 600;
  color: var(--text-main);
  font-size: 0.95rem;
}

.user-role {
  font-size: 0.8rem;
  color: #94a3b8;
}

.dropdown-divider {
  height: 1px;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.1) 10%, rgba(255, 255, 255, 0.1) 90%, transparent);
  margin: 0.25rem 0;
}

.dropdown-item {
  display: flex;
  align-items: center;
  gap: 0.8rem;
  padding: 0.8rem 1rem;
  border-radius: 10px;
  cursor: pointer;
  color: var(--text-main);
  font-weight: 500;
  transition: all 0.2s ease;
}

.dropdown-item:hover {
  background: rgba(255, 255, 255, 0.08);
  transform: translateX(2px);
}

.dropdown-item .icon {
  font-size: 1.1rem;
  opacity: 0.8;
}

.text-danger {
  color: #fb7185 !important;
}

.text-danger:hover {
  background: rgba(251, 113, 133, 0.1) !important;
  color: #f43f5e !important;
}


</style>