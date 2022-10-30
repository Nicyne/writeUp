import { render, screen } from '@testing-library/react';
import { App } from 'app';
import { MemoryRouter } from 'react-router-dom';

test('renders learn react link', () => {
  render(
    <MemoryRouter initialEntries={['/']}>
      <App />
    </MemoryRouter>
  );
  const linkElement = screen.getByText(/learn react/i);
  expect(linkElement).toBeInTheDocument();
});

test('no match route works correctly', () => {
  render(
    <MemoryRouter initialEntries={['/invalid-route']}>
      <App />
    </MemoryRouter>
  );
  const linkElement = screen.getByText(/Not Found/i);
  expect(linkElement).toBeInTheDocument();
});
