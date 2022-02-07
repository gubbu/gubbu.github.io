"use strict";(self.webpackChunk_JUPYTERLAB_CORE_OUTPUT=self.webpackChunk_JUPYTERLAB_CORE_OUTPUT||[]).push([[4925],{84925:(e,t,o)=>{o.r(t),o.d(t,{default:()=>W});var n=o(50586),a=o(88904),r=o(30276),i=o(71439),s=o(62069),l=o(74165),d=o(86361),c=o(21586),p=o(24239),u=o(28013),m=o(16264),g=o(93315),b=o(79444),h=o(18151);const v=new RegExp("/(notebooks|edit)/(.*)"),x=/\.ipynb$/;var f;!function(e){e.toggleTop="application:toggle-top",e.toggleZen="application:toggle-zen",e.openLab="application:open-lab",e.openTree="application:open-tree",e.rename="application:rename",e.resolveTree="application:resolve-tree"}(f||(f={}));const w={id:"@retrolab/application-extension:dirty",autoStart:!0,requires:[n.ILabStatus,p.ITranslator],activate:(e,t,o)=>{if(!(e instanceof u.RetroApp))throw new Error(`${w.id} must be activated in RetroLab.`);const n=o.load("retrolab").__("Are you sure you want to exit RetroLab?\n\nAny unsaved changes will be lost.");window.addEventListener("beforeunload",(t=>{if(e.status.isDirty)return t.returnValue=n}))}},I={id:"@retrolab/application-extension:logo",autoStart:!0,activate:e=>{const t=i.PageConfig.getBaseUrl(),o=document.createElement("a");o.href=`${t}retro/tree`,o.target="_blank",o.rel="noopener noreferrer";const n=new h.Widget({node:o});("true"===i.PageConfig.getOption("retroLogo")?m.retroInlineIcon:m.jupyterIcon).element({container:o,elementPosition:"center",padding:"2px 2px 2px 8px",height:"28px",width:"auto"}),n.id="jp-RetroLogo",e.shell.add(n,"top",{rank:0})}},C={id:"@retrolab/application-extension:opener",autoStart:!0,requires:[n.IRouter,s.IDocumentManager],activate:(e,t,o)=>{const{commands:n}=e,a="router:tree";n.addCommand(a,{execute:t=>{var n;const a=null!==(n=t.path.match(v))&&void 0!==n?n:[],[,,r]=a;if(!r)return;const s=decodeURIComponent(r),l=i.PathExt.extname(s);e.restored.then((()=>{".ipynb"===l?o.open(s,"Notebook",void 0,{ref:"_noref"}):o.open(s,"Editor",void 0,{ref:"_noref"})}))}}),t.register({command:a,pattern:v})}},R={id:"@retrolab/application-extension:menus",requires:[d.IMainMenu],autoStart:!0,activate:(e,t)=>{switch(t.tabsMenu.dispose(),i.PageConfig.getOption("retroPage")){case"consoles":case"terminals":case"tree":t.editMenu.dispose(),t.kernelMenu.dispose(),t.runMenu.dispose();break;case"edit":t.kernelMenu.dispose(),t.runMenu.dispose()}}},S={id:"@retrolab/application-extension:pages",autoStart:!0,requires:[p.ITranslator],optional:[a.ICommandPalette,d.IMainMenu],activate:(e,t,o,n)=>{const a=t.load("retrolab"),r=i.PageConfig.getBaseUrl();e.commands.addCommand(f.openLab,{label:a.__("Open JupyterLab"),execute:()=>{window.open(`${r}lab`)}}),e.commands.addCommand(f.openTree,{label:a.__("Open Files"),execute:()=>{window.open(`${r}retro/tree`)}}),o&&[f.openLab,f.openTree].forEach((e=>{o.addItem({command:e,category:"View"})})),n&&n.viewMenu.addGroup([{command:f.openLab},{command:f.openTree}],0)}},E={id:"@retrolab/application-extension:paths",autoStart:!0,provides:n.JupyterFrontEnd.IPaths,activate:e=>{if(!(e instanceof u.RetroApp))throw new Error(`${E.id} must be activated in RetroLab.`);return e.paths}},P={id:"@retrolab/application-extension:router",autoStart:!0,provides:n.IRouter,requires:[n.JupyterFrontEnd.IPaths],activate:(e,t)=>{const{commands:o}=e,a=t.urls.base,r=new n.Router({base:a,commands:o});return e.started.then((()=>{r.route(),window.addEventListener("popstate",(()=>{r.route()}))})),r}},T={id:"@retrolab/application-extension:session-dialogs",provides:a.ISessionContextDialogs,autoStart:!0,activate:()=>a.sessionContextDialogs},M={id:"@retrolab/application-extension:shell",activate:e=>{if(!(e.shell instanceof u.RetroShell))throw new Error(`${M.id} did not find a RetroShell instance.`);return e.shell},autoStart:!0,provides:u.IRetroShell},L={id:"@retrolab/application-extension:spacer",autoStart:!0,activate:e=>{const t=new h.Widget;t.id=a.DOMUtils.createDomID(),t.addClass("jp-RetroSpacer"),e.shell.add(t,"top",{rank:1e4});const o=new h.Widget;o.id=a.DOMUtils.createDomID(),o.addClass("jp-RetroSpacer"),e.shell.add(o,"menu",{rank:1e4})}},y={id:"@retrolab/application-extension:status",autoStart:!0,provides:n.ILabStatus,activate:e=>{if(!(e instanceof u.RetroApp))throw new Error(`${y.id} must be activated in RetroLab.`);return e.status}},_={id:"@retrolab/application-extension:tab-title",autoStart:!0,requires:[u.IRetroShell],activate:(e,t)=>{const o=()=>{const e=t.currentWidget;if(e instanceof r.ConsolePanel){const t=()=>{const t=e.sessionContext.path||e.sessionContext.name,o=i.PathExt.basename(t);document.title=o.replace(x,"")};return e.sessionContext.sessionChanged.connect(t),void t()}if(e instanceof l.DocumentWidget){const t=()=>{const t=i.PathExt.basename(e.context.path);document.title=t.replace(x,"")};e.context.pathChanged.connect(t),t()}};t.currentChanged.connect(o),o()}},D={id:"@retrolab/application-extension:title",autoStart:!0,requires:[u.IRetroShell,p.ITranslator],optional:[s.IDocumentManager,n.IRouter],activate:(e,t,o,n,a)=>{const{commands:r}=e,d=o.load("retrolab"),c=new h.Widget;c.id="jp-title",e.shell.add(c,"top",{rank:10});const p=async()=>{const e=t.currentWidget;if(!(e&&e instanceof l.DocumentWidget))return;if(c.node.children.length>0)return;const o=document.createElement("h1");if(o.textContent=e.title.label.replace(x,""),c.node.appendChild(o),c.node.style.marginLeft="10px",!n)return;const p=()=>{const{currentWidget:e}=t;return!(!e||!n.contextForWidget(e))};r.addCommand(f.rename,{label:()=>d.__("Rename…"),isEnabled:p,execute:async()=>{var t,r;if(!p())return;const l=await(0,s.renameDialog)(n,e.context.path);if(e&&e.activate(),null===l)return;const d=null!==(t=e.context.path)&&void 0!==t?t:l.path,c=i.PathExt.basename(d);if(o.textContent=c.replace(x,""),!a)return;const u=null!==(r=a.current.path.match(v))&&void 0!==r?r:[],[,m,g]=u;if(!m||!g)return;const b=encodeURIComponent(d);a.navigate(`/retro/${m}/${b}`,{skipRouting:!0})}}),c.node.onclick=async()=>{r.execute(f.rename)}};t.currentChanged.connect(p),p()}},k={id:"@retrolab/application-extension:top",requires:[u.IRetroShell,p.ITranslator],optional:[d.IMainMenu,c.ISettingRegistry],activate:(e,t,o,n,a)=>{const r=o.load("retrolab"),i=t.top,s=k.id;e.commands.addCommand(f.toggleTop,{label:r.__("Show Header"),execute:()=>{i.setHidden(i.isVisible),a&&a.set(s,"visible",i.isVisible)},isToggled:()=>i.isVisible}),n&&n.viewMenu.addGroup([{command:f.toggleTop}],2);let l=!1;if(a){const t=a.load(s),o=e=>{const t=e.get("visible").composite;void 0!==e.user.visible&&(l=!0,i.setHidden(!t))};Promise.all([t,e.restored]).then((([e])=>{o(e),e.changed.connect((e=>{o(e)}))})).catch((e=>{console.error(e.message)}))}e.formatChanged.connect((()=>{l||("desktop"===e.format?t.expandTop():t.collapseTop())}))},autoStart:!0},U={id:"@retrolab/application-extension:tree-resolver",autoStart:!0,requires:[n.IRouter],provides:n.JupyterFrontEnd.ITreeResolver,activate:(e,t)=>{const{commands:o}=e,n=new b.DisposableSet,a=new g.PromiseDelegate,r=new RegExp("/retro(/tree/.*)?");n.add(o.addCommand(f.resolveTree,{execute:async e=>{var t;if(n.isDisposed)return;const o=i.URLExt.queryStringToObject(null!==(t=e.search)&&void 0!==t?t:""),r=o["file-browser-path"]||"";delete o["file-browser-path"],n.dispose(),a.resolve({browser:r,file:i.PageConfig.getOption("treePath")})}})),n.add(t.register({command:f.resolveTree,pattern:r}));const s=()=>{n.isDisposed||(n.dispose(),a.resolve(null))};return t.routed.connect(s),n.add(new b.DisposableDelegate((()=>{t.routed.disconnect(s)}))),{paths:a.promise}}},O={id:"@retrolab/application-extension:tree-updater",requires:[n.IRouter],provides:n.ITreePathUpdater,activate:(e,t)=>function(e){if(e!==i.PageConfig.getOption("treePath")){const o=i.URLExt.join(i.PageConfig.getOption("baseUrl")||"/","retro","tree",i.URLExt.encodeParts(e));t.navigate(o,{skipRouting:!0}),i.PageConfig.setOption("treePath",e)}},autoStart:!0},q={id:"@retrolab/application-extension:zen",autoStart:!0,requires:[p.ITranslator],optional:[a.ICommandPalette,u.IRetroShell,d.IMainMenu],activate:(e,t,o,n,a)=>{const{commands:r}=e,i=document.documentElement,s=t.load("retrolab"),l=()=>{null==n||n.expandTop(),null==n||n.menu.setHidden(!1),d=!1};let d=!1;r.addCommand(f.toggleZen,{label:s.__("Toggle Zen Mode"),execute:()=>{d?(document.exitFullscreen(),l()):(i.requestFullscreen(),null==n||n.collapseTop(),null==n||n.menu.setHidden(!0),d=!0)}}),document.addEventListener("fullscreenchange",(()=>{document.fullscreenElement||l()})),o&&o.addItem({command:f.toggleZen,category:"Mode"}),a&&a.viewMenu.addGroup([{command:f.toggleZen}],3)}},W=[w,I,R,C,S,E,P,T,M,L,y,_,D,k,U,O,q]}}]);
//# sourceMappingURL=4925.1e2b12a4f6f456cae2b1.js.map