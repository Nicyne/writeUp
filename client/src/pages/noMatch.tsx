export function NoMatch() {
  return (
    <div className="container">
      <div className="center">
        <h1>
          Page <span>{window.location.pathname}</span> not found
        </h1>
      </div>
    </div>
  );
}
