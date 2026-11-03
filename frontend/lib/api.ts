import type {
  Answer,
  ExamAttemptSummary,
  ExamDetail,
  LabResponse,
  LessonDetail,
  LessonListResponse,
  ProgressResponse,
  SubmitResponse,
} from "./types";

const API_URL = process.env.NEXT_PUBLIC_API_URL ?? "http://127.0.0.1:3001";

export class ApiError extends Error {
  status: number;
  code: string;
  details?: string;

  constructor(status: number, code: string, message: string, details?: string) {
    super(message);
    this.name = "ApiError";
    this.status = status;
    this.code = code;
    this.details = details;
  }
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  let response: Response;
  try {
    response = await fetch(`${API_URL}${path}`, {
      ...init,
      headers: {
        ...(init?.body ? { "Content-Type": "application/json" } : {}),
        ...init?.headers,
      },
    });
  } catch {
    throw new ApiError(
      0,
      "network",
      "Das Backend ist nicht erreichbar. Starte die API auf http://127.0.0.1:3001.",
    );
  }

  const text = await response.text();
  let data: unknown = null;
  if (text) {
    try {
      data = JSON.parse(text) as unknown;
    } catch {
      throw new ApiError(response.status, "invalid_response", "Die API hat kein JSON geliefert.");
    }
  }

  if (!response.ok) {
    const body = data as { error?: { code?: string; message?: string; details?: string } } | null;
    throw new ApiError(
      response.status,
      body?.error?.code ?? "http_error",
      body?.error?.message ?? "Die Anfrage ist fehlgeschlagen.",
      body?.error?.details,
    );
  }
  return data as T;
}

export function getLessons() {
  return request<LessonListResponse>("/api/v1/lessons");
}

export function getLesson(id: string) {
  return request<LessonDetail>(`/api/v1/lessons/${encodeURIComponent(id)}`);
}

export function getProgress() {
  return request<ProgressResponse>("/api/v1/progress");
}

export function resetProgress() {
  return request<{ reset: boolean }>("/api/v1/progress", { method: "DELETE" });
}

export function completeLesson(id: string) {
  return request<{ lesson_id: string; completed: boolean }>(
    `/api/v1/lessons/${encodeURIComponent(id)}/complete`,
    { method: "POST" },
  );
}

export function submitExercise(id: string, answer: Answer) {
  return request<SubmitResponse>(`/api/v1/exercises/${encodeURIComponent(id)}/submit`, {
    method: "POST",
    body: JSON.stringify({ answer }),
  });
}

export function evaluateLab(a: string, b: string) {
  return request<LabResponse>("/api/v1/lab/evaluate", {
    method: "POST",
    body: JSON.stringify({ a, b }),
  });
}

export function startExam() {
  return request<ExamDetail>("/api/v1/exam/start", { method: "POST" });
}

export function listExamAttempts() {
  return request<ExamAttemptSummary[]>("/api/v1/exam/attempts");
}

export function getExam(id: string) {
  return request<ExamDetail>(`/api/v1/exam/${encodeURIComponent(id)}`);
}

export function submitExam(id: string, answers: { exercise_id: string; answer: Answer }[]) {
  return request<ExamDetail>(`/api/v1/exam/${encodeURIComponent(id)}/submit`, {
    method: "POST",
    body: JSON.stringify({ answers }),
  });
}

export function errorText(error: unknown): string {
  if (error instanceof ApiError) {
    return error.details ? `${error.message} ${error.details}` : error.message;
  }
  return "Etwas ist schiefgelaufen.";
}
