"use strict";(self.webpackChunk_JUPYTERLAB_CORE_OUTPUT=self.webpackChunk_JUPYTERLAB_CORE_OUTPUT||[]).push([[9458,5018],{65018:(e,t,n)=>{n.r(t),n.d(t,{default:()=>u});var r,o=n(50586),a=n(88904),d=n(71439),i=n(46534),c=n(17682),s=n(21586),w=n(24239);!function(e){e.markdownPreview="markdownviewer:open",e.markdownEditor="markdownviewer:edit"}(r||(r={}));const m="Markdown Preview",p={activate:function(e,t,n,o,s){const w=n.load("jupyterlab"),{commands:u,docRegistry:l}=e;t.addFactory(c.markdownRendererFactory);const k=new a.WidgetTracker({namespace:"markdownviewer-widget"});let g=Object.assign({},i.MarkdownViewer.defaultConfig);function h(e){Object.keys(g).forEach((t=>{var n;e.setOption(t,null!==(n=g[t])&&void 0!==n?n:null)}))}if(s){const e=e=>{g=e.composite,k.forEach((e=>{h(e.content)}))};s.load(p.id).then((t=>{t.changed.connect((()=>{e(t)})),e(t)})).catch((e=>{console.error(e.message)}))}const f=new i.MarkdownViewerFactory({rendermime:t,name:m,primaryFileType:l.getFileType("markdown"),fileTypes:["markdown"],defaultRendered:["markdown"]});return f.widgetCreated.connect(((e,t)=>{t.context.pathChanged.connect((()=>{k.save(t)})),h(t.content),k.add(t)})),l.addWidgetFactory(f),o&&o.restore(k,{command:"docmanager:open",args:e=>({path:e.context.path,factory:m}),name:e=>e.context.path}),u.addCommand(r.markdownPreview,{label:w.__("Markdown Preview"),execute:e=>{const t=e.path;if("string"==typeof t)return u.execute("docmanager:open",{path:t,factory:m,options:e.options})}}),u.addCommand(r.markdownEditor,{execute:()=>{const e=k.currentWidget;if(!e)return;const t=e.context.path;return u.execute("docmanager:open",{path:t,factory:"Editor",options:{mode:"split-right"}})},isVisible:()=>{const e=k.currentWidget;return e&&".md"===d.PathExt.extname(e.context.path)||!1},label:w.__("Show Markdown Editor")}),k},id:"@jupyterlab/markdownviewer-extension:plugin",provides:i.IMarkdownViewerTracker,requires:[c.IRenderMimeRegistry,w.ITranslator],optional:[o.ILayoutRestorer,s.ISettingRegistry],autoStart:!0},u=p}}]);
//# sourceMappingURL=9458.0ce669567b2b22b9b9e4.js.map