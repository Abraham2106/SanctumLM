import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import type { NotebookSummary } from "../ipc-types.ts";
import {
  ensureMockSeed,
  getIpcTransport,
  notebookCreate,
  notebookList,
} from "../lib/ipc-client.ts";
import { formatProductError } from "../lib/product-errors.ts";

export type NotebookContextValue = {
  notebooks: NotebookSummary[];
  activeNotebookId: string;
  setActiveNotebookId: (id: string) => void;
  activeNotebook: NotebookSummary | undefined;
  filter: string;
  setFilter: (value: string) => void;
  filteredNotebooks: NotebookSummary[];
  loading: boolean;
  error: string;
  clearError: () => void;
  refreshNotebooks: () => Promise<void>;
  createNotebook: (title: string) => Promise<boolean>;
};

const NotebookContext = createContext<NotebookContextValue | null>(null);

export function NotebookProvider({ children }: { children: ReactNode }) {
  const [notebooks, setNotebooks] = useState<NotebookSummary[]>([]);
  const [activeNotebookId, setActiveNotebookId] = useState("");
  const [filter, setFilter] = useState("");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  const refreshNotebooks = useCallback(async () => {
    setError("");
    let result = await notebookList();
    if (!result.ok) {
      setError(formatProductError(result.error));
      setLoading(false);
      return;
    }
    if (result.data.notebooks.length === 0 && getIpcTransport() === "mock") {
      ensureMockSeed();
      result = await notebookList();
      if (!result.ok) {
        setError(formatProductError(result.error));
        setLoading(false);
        return;
      }
    }
    setNotebooks(result.data.notebooks);
    setActiveNotebookId((current) => {
      if (current && result.ok && result.data.notebooks.some((n) => n.notebook_id === current)) {
        return current;
      }
      return result.ok && result.data.notebooks[0] ? result.data.notebooks[0].notebook_id : "";
    });
    setLoading(false);
  }, []);

  useEffect(() => {
    void refreshNotebooks();
  }, [refreshNotebooks]);

  const createNotebook = useCallback(
    async (title: string) => {
      const trimmed = title.trim();
      if (!trimmed) {
        return false;
      }
      setError("");
      const result = await notebookCreate(trimmed);
      if (!result.ok) {
        setError(formatProductError(result.error));
        return false;
      }
      setNotebooks((prev) => [...prev, result.data.notebook]);
      setActiveNotebookId(result.data.notebook.notebook_id);
      return true;
    },
    [],
  );

  const filteredNotebooks = useMemo(() => {
    const q = filter.trim().toLowerCase();
    if (!q) {
      return notebooks;
    }
    return notebooks.filter((nb) => nb.title.toLowerCase().includes(q));
  }, [filter, notebooks]);

  const activeNotebook = notebooks.find((n) => n.notebook_id === activeNotebookId);

  const value = useMemo(
    () => ({
      notebooks,
      activeNotebookId,
      setActiveNotebookId,
      activeNotebook,
      filter,
      setFilter,
      filteredNotebooks,
      loading,
      error,
      clearError: () => setError(""),
      refreshNotebooks,
      createNotebook,
    }),
    [
      notebooks,
      activeNotebookId,
      activeNotebook,
      filter,
      filteredNotebooks,
      loading,
      error,
      refreshNotebooks,
      createNotebook,
    ],
  );

  return <NotebookContext.Provider value={value}>{children}</NotebookContext.Provider>;
}

export function useNotebookContext(): NotebookContextValue {
  const ctx = useContext(NotebookContext);
  if (!ctx) {
    throw new Error("useNotebookContext debe usarse dentro de NotebookProvider");
  }
  return ctx;
}
