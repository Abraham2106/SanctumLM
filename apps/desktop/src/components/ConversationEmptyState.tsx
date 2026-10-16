import type { ChatMode } from "../ipc-types.ts";

type ConversationEmptyStateProps = {
  mode: ChatMode;
  hasActiveNotebook: boolean;
};

export default function ConversationEmptyState({
  mode,
  hasActiveNotebook,
}: ConversationEmptyStateProps) {
  if (mode === "notebook" && !hasActiveNotebook) {
    return (
      <div className="empty-state empty-state-chat" aria-labelledby="chat-empty-no-nb">
        <h4 id="chat-empty-no-nb">Sin cuaderno para el chat contextual</h4>
        <p>
          Elige o crea un cuaderno en la barra superior para preguntar sobre sus fuentes
          indexadas (R-PROD-006).
        </p>
        <p className="placeholder-note">
          También puedes cambiar a <strong>General</strong> para hablar con el modelo sin contexto
          de documentos (R-PROD-007).
        </p>
      </div>
    );
  }

  if (mode === "general") {
    return (
      <div className="empty-state empty-state-chat" aria-labelledby="chat-empty-general">
        <h4 id="chat-empty-general">Conversación general vacía</h4>
        <p>
          Escribe abajo para iniciar. El núcleo no recuperará fragmentos de ningún cuaderno
          (R-PROD-007).
        </p>
        <p className="placeholder-note">
          Elige el modelo local y pulsa <strong>Enviar</strong> o Enter (Shift+Enter para nueva
          línea).
        </p>
      </div>
    );
  }

  return (
    <div className="empty-state empty-state-chat" aria-labelledby="chat-empty-notebook">
      <h4 id="chat-empty-notebook">Aún no hay mensajes en este cuaderno</h4>
      <p>
        Pregunta sobre las fuentes indexadas del cuaderno activo. Las respuestas del asistente
        pueden incluir citas verificadas a fragmentos FTS (R-PROD-006).
      </p>
      <p className="placeholder-note">
        Si no hay fuentes listas, importa TXT o MD en el panel Fuentes antes de chatear.
      </p>
    </div>
  );
}
