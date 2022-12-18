import { Link } from 'react-router-dom';
import { useAuth } from 'hooks';

export function PageHeader() {
  const { user } = useAuth();

  return (
    <header>
      <nav>
        <ul>
          <li>
            <Link to="/">writeUp</Link>
          </li>
          <li>
            <Link to="/app">App</Link>
          </li>
          {user ? (
            <>
              <li>{user.username}</li>
              <li>
                <Link to="/logout">logout</Link>
              </li>
            </>
          ) : (
            <>
              <li>
                <Link to="/login">login</Link>
              </li>
              <li>
                <Link to="/signup">signup</Link>
              </li>
            </>
          )}
        </ul>
      </nav>
    </header>
  );
}
