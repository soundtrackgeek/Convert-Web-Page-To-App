import { useState } from 'react';
import './App.css';

function App() {
  const [url, setUrl] = useState('');

  const handleSubmit = (e) => {
    e.preventDefault();
    // TODO: Implement webpage conversion logic
    console.log('Converting URL:', url);
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
    </div>
  );
}

export default App;
