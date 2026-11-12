export function isPointerActivation(event: Event): boolean {
  return (typeof PointerEvent !== "undefined" && event instanceof PointerEvent && !!event.pointerType)
    || (typeof TouchEvent !== "undefined" && event instanceof TouchEvent)
    || (event instanceof MouseEvent && (event.detail > 0 || event.type === "mousedown" || event.type === "mouseup"));
}
