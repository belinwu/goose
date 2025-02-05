"use strict";(self.webpackChunkgoose=self.webpackChunkgoose||[]).push([[8848],{4435:(e,s,n)=>{n.r(s),n.d(s,{assets:()=>l,contentTitle:()=>a,default:()=>h,frontMatter:()=>r,metadata:()=>i,toc:()=>d});const i=JSON.parse('{"id":"guides/using-goosehints","title":"Using Goosehints","description":".goosehints is a text file used to provide additional context about your project and improve the communication with Goose. The use of goosehints ensures that Goose understands your requirements better and can execute tasks more effectively.","source":"@site/docs/guides/using-goosehints.md","sourceDirName":"guides","slug":"/guides/using-goosehints","permalink":"/goose/pr-preview/pr-1070/docs/guides/using-goosehints","draft":false,"unlisted":false,"tags":[],"version":"current","sidebarPosition":3,"frontMatter":{"title":"Using Goosehints","sidebar_position":3},"sidebar":"tutorialSidebar","previous":{"title":"CLI Commands","permalink":"/goose/pr-preview/pr-1070/docs/guides/goose-cli-commands"},"next":{"title":"LLM Rate Limits","permalink":"/goose/pr-preview/pr-1070/docs/guides/handling-llm-rate-limits-with-goose"}}');var t=n(4848),o=n(8453);const r={title:"Using Goosehints",sidebar_position:3},a="Providing Hints to Goose",l={},d=[{value:"Creating your hints file",id:"creating-your-hints-file",level:2},{value:"Setting up hints",id:"setting-up-hints",level:2},{value:"Best practices",id:"best-practices",level:2}];function c(e){const s={a:"a",admonition:"admonition",code:"code",h1:"h1",h2:"h2",header:"header",li:"li",p:"p",pre:"pre",strong:"strong",ul:"ul",...(0,o.R)(),...e.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(s.header,{children:(0,t.jsx)(s.h1,{id:"providing-hints-to-goose",children:"Providing Hints to Goose"})}),"\n",(0,t.jsxs)(s.p,{children:[(0,t.jsx)(s.code,{children:".goosehints"})," is a text file used to provide additional context about your project and improve the communication with Goose. The use of ",(0,t.jsx)(s.code,{children:"goosehints"})," ensures that Goose understands your requirements better and can execute tasks more effectively."]}),"\n",(0,t.jsx)(s.admonition,{title:"Developer extension required",type:"info",children:(0,t.jsxs)(s.p,{children:["To make use of the hints file, you need to have the ",(0,t.jsx)(s.code,{children:"Developer"})," extension ",(0,t.jsx)(s.a,{href:"/docs/getting-started/using-extensions",children:"enabled"}),"."]})}),"\n",(0,t.jsxs)(s.p,{children:["This guide will walk you through creating and using your ",(0,t.jsx)(s.code,{children:".goosehints"})," file to streamline your workflow with custom instructions and context."]}),"\n",(0,t.jsx)(s.h2,{id:"creating-your-hints-file",children:"Creating your hints file"}),"\n",(0,t.jsxs)(s.p,{children:["Create a file named ",(0,t.jsx)(s.code,{children:".goosehints"})," and save the file in ",(0,t.jsx)(s.code,{children:"~/.config/goose/.goosehints"}),". If saved here, Goose will use this file for every session with you."]}),"\n",(0,t.jsx)(s.admonition,{type:"tip",children:(0,t.jsxs)(s.p,{children:["You can also save ",(0,t.jsx)(s.code,{children:".goosehints"})," local to any directory. In this case, Goose will utilize the hints when working in that directory."]})}),"\n",(0,t.jsxs)(s.p,{children:["The ",(0,t.jsx)(s.code,{children:".goosehints"})," file can include any instructions or contextual details relevant to your projects."]}),"\n",(0,t.jsxs)(s.p,{children:["A good time to consider adding a ",(0,t.jsx)(s.code,{children:".goosehints"})," file is when you find yourself repeating prompts, or providing the same kind of instructions multiple times. It's also a great way to provide a lot of context which might be better suited in a file."]}),"\n",(0,t.jsx)(s.h2,{id:"setting-up-hints",children:"Setting up hints"}),"\n",(0,t.jsxs)(s.p,{children:["The ",(0,t.jsx)(s.code,{children:".goosehints"})," file supports natural language and also follows ",(0,t.jsx)(s.a,{href:"https://jinja.palletsprojects.com/en/stable/",children:"jinja templating rules"}),", so you can leverage templating to insert file contents or variables."]}),"\n",(0,t.jsx)(s.p,{children:"Here are some ways people have used hints to provide additional context to Goose:"}),"\n",(0,t.jsxs)(s.ul,{children:["\n",(0,t.jsxs)(s.li,{children:["\n",(0,t.jsxs)(s.p,{children:[(0,t.jsx)(s.strong,{children:"Decision-Making"}),": Specify if Goose should autonomously make changes or confirm actions with you first."]}),"\n"]}),"\n",(0,t.jsxs)(s.li,{children:["\n",(0,t.jsxs)(s.p,{children:[(0,t.jsx)(s.strong,{children:"Validation Routines"}),": Provide test cases or validation methods that Goose should perform to ensure changes meet project specifications."]}),"\n"]}),"\n",(0,t.jsxs)(s.li,{children:["\n",(0,t.jsxs)(s.p,{children:[(0,t.jsx)(s.strong,{children:"Feedback Loop"}),": Include steps that allow Goose to receive feedback and iteratively improve its suggestions."]}),"\n"]}),"\n",(0,t.jsxs)(s.li,{children:["\n",(0,t.jsxs)(s.p,{children:[(0,t.jsx)(s.strong,{children:"Point to more detailed documentation"}),": Indicate important files like ",(0,t.jsx)(s.code,{children:"README.md"}),", ",(0,t.jsx)(s.code,{children:"CONTRIBUTING.md"}),", or others that Goose should consult for detailed explanations."]}),"\n"]}),"\n"]}),"\n",(0,t.jsxs)(s.p,{children:["Like prompts, this is not an extensive list to shape your ",(0,t.jsx)(s.code,{children:".goosehints"})," file. You can include as much context as you need."]}),"\n",(0,t.jsxs)(s.p,{children:["Example ",(0,t.jsx)(s.code,{children:".goosehints"})," file:"]}),"\n",(0,t.jsx)(s.pre,{children:(0,t.jsx)(s.code,{className:"language-jinja",children:"This is a simple example JavaScript web application that uses the Express.js framework. View [Express documentation](https://expressjs.com/) for extended guidance.\n\nGo through the README.md for information on how to build and test it as needed.\n\nMake sure to confirm all changes with me before applying.\n\nUse the following custom values when needed:\n{%include custom-config.js%}\n\nRun tests with `npm run test` ideally after each change.\n"})}),"\n",(0,t.jsx)(s.h2,{id:"best-practices",children:"Best practices"}),"\n",(0,t.jsxs)(s.ul,{children:["\n",(0,t.jsxs)(s.li,{children:[(0,t.jsx)(s.strong,{children:"Keep file updated"}),": Regularly update the ",(0,t.jsx)(s.code,{children:".goosehints"})," file to reflect any changes in project protocols or priorities."]}),"\n",(0,t.jsxs)(s.li,{children:[(0,t.jsx)(s.strong,{children:"Be concise"}),": Make sure the content is straightforward and to the point, ensuring Goose can quickly parse and act on the information."]}),"\n"]})]})}function h(e={}){const{wrapper:s}={...(0,o.R)(),...e.components};return s?(0,t.jsx)(s,{...e,children:(0,t.jsx)(c,{...e})}):c(e)}},8453:(e,s,n)=>{n.d(s,{R:()=>r,x:()=>a});var i=n(6540);const t={},o=i.createContext(t);function r(e){const s=i.useContext(o);return i.useMemo((function(){return"function"==typeof e?e(s):{...s,...e}}),[s,e])}function a(e){let s;return s=e.disableParentContext?"function"==typeof e.components?e.components(t):e.components||t:r(e.components),i.createElement(o.Provider,{value:s},e.children)}}}]);