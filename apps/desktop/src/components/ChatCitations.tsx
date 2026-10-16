import type { ChatCitation } from "../ipc-types.ts";

const VERIFICATION_ES: Record<ChatCitation["verification"], string> = {
  backed: "con respaldo",
  inference: "inferencia",
  discrepancy: "discrepancia",
  unverified: "sin verificar",
};

function citationQuotePreview(quote: string, maxLen = 72): string {
  const trimmed = quote.trim();
  if (trimmed.length <= maxLen) {
    return trimmed;
  }
  return `${trimmed.slice(0, maxLen)}…`;
}

function citationAccessibleName(citation: ChatCitation): string {
  const estado = VERIFICATION_ES[citation.verification];
  const preview = citationQuotePreview(citation.quote);
  const truncated = citation.quote.trim().length > 72;
  const quotePart = truncated
    ? `«${preview}» (texto completo disponible al activar la cita)`
    : `«${preview}»`;
  return `Abrir fragmento citado: ${quotePart}. Estado de verificación: ${estado}.`;
}

type ChatCitationsProps = {
  messageId: string;
  citations: ChatCitation[];
  onCitationActivate: (citation: ChatCitation) => void;
};

export default function ChatCitations({
  messageId,
  citations,
  onCitationActivate,
}: ChatCitationsProps) {
  if (citations.length === 0) {
    return null;
  }

  return (
    <section className="citation-block" aria-label="Citas de fuentes">
      <h4 className="citation-block-title" id={`citations-${messageId}`}>
        Citas
      </h4>
      <ul className="citation-list" aria-labelledby={`citations-${messageId}`}>
        {citations.map((citation) => (
          <li key={`${messageId}-${citation.chunk_id}`}>
            <button
              type="button"
              className="citation-link"
              aria-label={citationAccessibleName(citation)}
              onClick={() => onCitationActivate(citation)}
            >
              <span
                className={`citation-verification citation-verification-${citation.verification}`}
                aria-hidden="true"
              >
                {VERIFICATION_ES[citation.verification]}
              </span>
              <span className="citation-quote" aria-hidden="true">
                {citationQuotePreview(citation.quote)}
              </span>
            </button>
          </li>
        ))}
      </ul>
    </section>
  );
}
