// assets/scripts/regresa.ts
function regresa() {
  document.addEventListener("visibilitychange", () => {
    if (document.visibilityState === "hidden") {
      document.title = "Regresa please!";
    } else {
      document.title = "My Curriculum";
    }
  });
}
export {
  regresa
};
