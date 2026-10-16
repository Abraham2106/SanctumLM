import type { ProductError, ProductErrorCode } from "../ipc-types.ts";

const FALLBACK_MESSAGES: Record<ProductErrorCode, string> = {
  VALIDATION_FAILED: "Entrada no válida. Revisa los datos e inténtalo de nuevo.",
  UNSUPPORTED_API_VERSION: "Versión de API no compatible. Actualiza la aplicación.",
  POLICY_DENIED: "La política del cuaderno no permite esta acción.",
  NOT_FOUND: "No se encontró el recurso solicitado.",
  CONFLICT: "Conflicto con el estado actual. Actualiza la vista.",
  JOB_CANCELLED: "El trabajo fue cancelado.",
  LOCAL_UNAVAILABLE: "El núcleo local no está disponible.",
  SSR_BLOCKED: "La importación web fue bloqueada por seguridad.",
};

export function formatProductError(error: ProductError): string {
  const trimmed = error.message.trim();
  if (trimmed.length > 0) {
    return trimmed;
  }
  return FALLBACK_MESSAGES[error.code] ?? error.code;
}
