import { useState, useEffect } from 'react';
import { LoginScreen } from './components/LoginScreen';
import { MainLayout } from './components/MainLayout';
import { api } from './lib/tauri';

type AppState = 'loading' | 'login' | 'main';

export default function App() {
  const [appState, setAppState] = useState<AppState>('loading');

  useEffect(() => {
    api.checkAuth()
      .then((isAuthed) => setAppState(isAuthed ? 'main' : 'login'))
      .catch(() => setAppState('login'));
  }, []);

  if (appState === 'loading') {
    return (
      <div style={{
        height: '100vh', display: 'flex', alignItems: 'center',
        justifyContent: 'center', color: 'var(--text-muted)',
        background: 'var(--bg)',
      }}>
        Loading…
      </div>
    );
  }

  if (appState === 'login') {
    return <LoginScreen onLoggedIn={() => setAppState('main')} />;
  }

  return (
    <MainLayout
      onLogout={() => setAppState('login')}
    />
  );
}
