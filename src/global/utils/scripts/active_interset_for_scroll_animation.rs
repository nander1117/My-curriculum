use dioxus::prelude::*;

/// * ### Add intersection observer to elements with class intersect for scroll animation in tailwindcss
///   more info: [tailwindcss-intersect](https://github.com/heidkaemper/tailwindcss-intersect)
///
///   it's equivalent to adding the following script in html:
///
///   ```html
///   <script src="https://unpkg.com/tailwindcss-intersect@2.x.x/dist/observer.min.js" defer></script>
///   ```
pub fn active_interset_for_scroll_animation() {
    document::eval(
        r#"
            (()=>{var o={start(){if(document.readyState==="loading"){document.addEventListener("DOMContentLoaded",()=>this.observe());return}this.observe()},restart(){this._observers.forEach(s=>s.disconnect()),this._observers=[],this.observe()},observe(){let s=['[class*=" intersect:"]','[class*=":intersect:"]','[class^="intersect:"]','[class="intersect"]','[class*=" intersect "]','[class^="intersect "]','[class$=" intersect"]'];document.querySelectorAll(s.join(",")).forEach(e=>{let t=new IntersectionObserver(c=>{c.forEach(n=>{if(!n.isIntersecting){e.setAttribute("no-intersect","");return}e.removeAttribute("no-intersect"),e.classList.contains("intersect-once")&&t.disconnect()})},{threshold:this._getThreshold(e)});t.observe(e),this._observers.push(t)})},_getThreshold(s){return s.classList.contains("intersect-full")?.99:s.classList.contains("intersect-half")?.5:0},_observers:[]},r=o;r.start();})();
        "#,
    );
}
