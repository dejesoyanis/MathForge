import { ExamSession } from "@/components/ExamSession";

export default async function ExamAttemptPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = await params;
  return <ExamSession id={id} />;
}
