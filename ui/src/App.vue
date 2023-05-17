<template>
  <header>
    <div class="navbar bg-base-300">
      <div class="flex-1">
        <a class="btn btn-ghost normal-case text-xl">Virtman</a>
        <ul class="menu menu-horizontal px-1">
          <li><router-link to="/machine" activeClass="active">VMs</router-link></li>
          <li><router-link to="/network" activeClass="active">Networks</router-link></li>
          <li><router-link to="/storage" activeClass="active">Storage</router-link></li>
        </ul>
      </div>
      <!--<div class="flex-none">
        
      </div>-->
    </div>
  </header>

  <main>
    <router-view></router-view>
  </main>
</template>
