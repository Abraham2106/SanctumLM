# ADR-0005 — Gates y autoridad humana

- **Estado:** proposed
- **Fecha:** 2026-10-15
- **Alcance:** gobierno del desarrollo
- **Vínculos:** R-ENG-008, R-NFR-005, R-NFR-007; WBS-1.4, WBS-4.5

## Contexto

Un agente puede generar un plan convincente sin autorización para elegir producto, gastar, desplegar o publicar información.

## Decisión propuesta

G0 aprueba producto; G1 arquitectura; G2 vertical slice; G3 seguridad/calidad; G4 release. Operaciones irreversibles, permisos nuevos, exfiltración de datos, cambios de licencia y costos requieren aprobación humana específica, incluso si una etapa general fue aprobada. La revisión por otro agente nunca equivale a permiso del propietario.

## Alternativas

Autonomía sin límites: riesgo alto de seguridad y desalineación. Aprobación humana por cada línea de código: excesivo costo de coordinación.

## Consecuencias

El flujo puede bloquearse a la espera de decisión; registrar claramente el bloqueo y las opciones, sin simular autorización.
