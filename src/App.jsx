import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';

function App() {
  const [url, setUrl] = useState('');
  const [message, setMessage] = useState('');
  const [error, setError] = useState('');

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      setError('');
      setMessage('Converting webpage...');
      
      const response = await invoke('convert_webpage', {
        request: { url }
      });
      
      setMessage(response);
    } catch (err) {
      setError(err.toString());
      setMessage('');
    }
  };

  return (
    <div className="container">
      <h1>Webpage to App Converter</h1>
      <form onSubmit={handleSubmit}>
        <input
          type="url"
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          placeholder="Enter webpage URL"
          required
        />
        <button type="submit">Convert to App</button>
      </form>
      {message && <p className="message">{message}</p>}
      {error && <p className="error">{error}</p>}
    </div>
  );
}

export default App;
