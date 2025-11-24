function Task() {
  return <div>Task</div>;
}

export function Plan() {
  return (
    <div className="text-3xl font-bold">
      Plan View
      <Task />
    </div>
  );
}
