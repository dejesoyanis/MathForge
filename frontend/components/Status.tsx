export function Loading({ label }: { label: string }) {
  return (
    <p className="text-muted" role="status">
      {label}
    </p>
  );
}

export function ErrorNote({ message }: { message: string }) {
  return (
    <p className="rounded-md border border-bad/30 bg-white px-3 py-2 text-bad" role="alert">
      {message}
    </p>
  );
}
