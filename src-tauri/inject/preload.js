let isScrolling = false;
let scrollSpeed = 1;
let scrollTimer;

if (window.location.origin === "https://weread.qq.com") {
  console.log("hello world from js init script");
}

function getCurrentMaxWidth(element) {
  let currentValue = window.getComputedStyle(element).maxWidth;
  currentValue = currentValue.substring(0, currentValue.indexOf("px"));
  currentValue = parseInt(currentValue);
  return currentValue;
}

function changeWidth(increse) {
  const step = 100;
  const item1 = document.querySelector(".readerContent .app_content");
  const item2 = document.querySelector(".readerTopBar");
  const currentValue = getCurrentMaxWidth(item1);
  const changedValue = currentValue + (increse ? 1 : -1) * step;

  item1.style["max-width"] = changedValue + "px";
  item2.style["max-width"] = changedValue + "px";
  const event = new Event("resize");
  window.dispatchEvent(event);
}

const html = (strings, ...placeholders) => {
  return String.raw(strings, ...placeholders);
};

window.addEventListener("load", async (_event) => {
  const { invoke } = window.__TAURI__.core;
  console.log("execute onload...");

  const store = new Store("settings.json");
  isScrolling = (await store.get("is-scrolling")) ?? false;
  scrollSpeed = (await store.get("scroll-speed")) ?? 1;

  // 添加内容
  const btnControls = html`
    <button
      id="widthIncrease"
      title="加宽"
      class="readerControls_item extra-item"
      style="color:#6a6c6c;cursor:pointer;"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="icon icon-tabler icons-tabler-outline icon-tabler-viewport-wide"
      >
        <path stroke="none" d="M0 0h24v24H0z" fill="none" />
        <path d="M10 12h-7l3 -3m0 6l-3 -3" />
        <path d="M14 12h7l-3 -3m0 6l3 -3" />
        <path d="M3 6v-3h18v3" />
        <path d="M3 18v3h18v-3" />
      </svg>
    </button>
    <button
      id="widthDecrease"
      title="减宽"
      class="readerControls_item extra-item"
      style="color:#6a6c6c;cursor:pointer;"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="icon icon-tabler icons-tabler-outline icon-tabler-viewport-narrow"
      >
        <path stroke="none" d="M0 0h24v24H0z" fill="none" />
        <path d="M3 12h7l-3 -3m0 6l3 -3" />
        <path d="M21 12h-7l3 -3m0 6l-3 -3" />
        <path d="M9 6v-3h6v3" />
        <path d="M9 18v3h6v-3" />
      </svg>
    </button>

    <button
      id="controlScroll"
      title="滚动"
      class="readerControls_item extra-item"
      style="color:#6a6c6c;cursor:pointer;"
    >
      <svg
        id="controlScroll__start"
        style="display:block;"
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="icon icon-tabler icons-tabler-outline icon-tabler-square-chevrons-down"
      >
        <path stroke="none" d="M0 0h24v24H0z" fill="none" />
        <path d="M15 8l-3 3l-3 -3" />
        <path d="M15 13l-3 3l-3 -3" />
        <path
          d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14z"
        />
      </svg>
      <svg
        id="controlScroll__stop"
        style="display:none;"
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="icon icon-tabler icons-tabler-outline icon-tabler-player-pause"
      >
        <path stroke="none" d="M0 0h24v24H0z" fill="none" />
        <path
          d="M6 5m0 1a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v12a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1z"
        />
        <path
          d="M14 5m0 1a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v12a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1z"
        />
      </svg>
    </button>

    <button
      id="autoScroll"
      title="滚动速度"
      class="readerControls_item extra-item"
      style="color:#6a6c6c;cursor:pointer;font-size:14px;display:none;"
    >
      ×${scrollSpeed}
    </button>
  `;

  const btnSetting = html`
    <span class="readerTopBar_link_sep"></span>
    <a id="__setting__" class="readerTopBar_link">设置</a>
  `;

  const btnSetting1 = html`
    <span class="navBar_separator"></span>
    <button id="__setting__" class="navBar_link">设置</button>
  `;

  const elements = [
    { className: ".readerControls", element: btnControls },
    { className: ".navBar_inner", element: btnSetting1 },
    { className: ".readerTopBar_right", element: btnSetting },
  ];

  // isHorizontalReader/isNormalReader
  const isNormalReader = document.querySelector(
    ".readerControls_item.isNormalReader"
  );
  if (isNormalReader) {
  elements.forEach(({ className, element }) => {
      document
        .querySelector(className)
        ?.insertAdjacentHTML("beforeend", element);
  });
  }

  // 添加监听
  const events = {
    __setting__: async () => {
      await invoke("create_setting", {});
    },
    widthIncrease: () => {
      changeWidth(true);
    },
    widthDecrease: () => {
      changeWidth(false);
    },
    controlScroll: () => {
    isScrolling = !isScrolling;
    if (isScrolling) {
      document.getElementById("autoScroll").style.display = "block";
      scrollTimer = setInterval(() => {
        let totalHeight = document.documentElement.scrollTop;
        let scrollHeight = document.body.scrollHeight;
        window.scrollBy(0, scrollSpeed);
        totalHeight += scrollSpeed;
        if (totalHeight >= scrollHeight) {
          clearInterval(scrollTimer);
        }
      }, 10);
      document.getElementById("autoScroll").style.display = "block";
      document.getElementById("controlScroll__start").style.display = "none";
      document.getElementById("controlScroll__stop").style.display = "block";
    } else {
      document.getElementById("autoScroll").style.display = "none";
      document.getElementById("controlScroll__start").style.display = "block";
      document.getElementById("controlScroll__stop").style.display = "none";
      clearInterval(scrollTimer);
    }
    store.set("is-scrolling", isScrolling);
    store.save();
    },
    autoScroll: () => {
    scrollSpeed++;
    if (scrollSpeed > 10) {
      scrollSpeed = 1;
    }
    document.getElementById("autoScroll").innerHTML = `×${scrollSpeed}`;
    store.set("scroll-speed", scrollSpeed);

    store.save();
    },
  };

  Object.entries(events).forEach(([id, func]) => {
    document.getElementById(id)?.addEventListener("click", func);
  });

  // 下划隐藏顶栏，上划显示顶栏
  let windowTop = 0;
  window.addEventListener("scroll", function () {
    let scrollS = window.scrollY;
    let selBtn = document.querySelector(".readerTopBar");
    const readerControl = document.querySelector(".readerControls");
    readerControl.addEventListener("mouseenter", () => {
      readerControl.style.opacity = 1;
    });

    readerControl.addEventListener("mouseleave", () => {
      readerControl.style.opacity = 0;
    });

    if (scrollS >= windowTop) {
      // 上划显示
      selBtn.style.opacity = 0;
      windowTop = scrollS;
    } else {
      // 下滑隐藏
      selBtn.style.opacity = 1;
      windowTop = scrollS;
    }
  });
});

class Store {
  constructor(path) {
    this.path = path;
    this.invoke = window.__TAURI__.core.invoke;
    this.listen = window.__TAURI__.event.listen;
  }
  async get(key) {
    return await this.invoke("plugin:store|get", {
      path: this.path,
      key,
    });
  }

  /**
   * Inserts a key-value pair into the store.
   *
   * @param key
   * @param value
   * @returns
   */
  async set(key, value) {
    return await this.invoke("plugin:store|set", {
      path: this.path,
      key,
      value,
    });
  }

  async save() {
    return await this.invoke("plugin:store|save", {
      path: this.path,
    });
  }

  async onChange(cb) {
    return await this.listen("store://change", (event) => {
      if (event.payload.path === this.path) {
        cb(event.payload.key, event.payload.value);
      }
    });
  }
}

function injectStyle(css) {
  const weReadStyleElement = document.createElement("style");
  weReadStyleElement.innerHTML = css;
  document.head.appendChild(weReadStyleElement);
  console.log("inject style:", css);
}

function fontFamilyCss(fontFamily) {
  return `
  *:not(pre) {
    font-family: ${fontFamily} !important;
  }`;
}

document.addEventListener("DOMContentLoaded", async (_event) => {
  const store = new Store("settings.json");
  const fontFamily = (await store.get("font-family")) ?? "霞鹜文楷";
  const cssSnippet = (await store.get("css-snippet")) ?? "";

  injectStyle(fontFamilyCss(fontFamily));
  injectStyle(cssSnippet);

  store.onChange((key, value) => {
    console.log(key, value);
    if (key == "font-family") {
      injectStyle(fontFamilyCss(value));
      location.reload();
    }
  });
});
