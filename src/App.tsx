import React, { useEffect } from 'react';
import { BrowserRouter } from 'react-router-dom';

export interface AppProps {
  title?: string;
}

const App: React.FC<AppProps> = (props) => {
  const { title } = props;

  useEffect(() => {
    if (title && document.title !== title) {
      document.title = title;
    }
  }, [title]);

  return (
    <BrowserRouter>
    </BrowserRouter>
  )
};
export default App;
