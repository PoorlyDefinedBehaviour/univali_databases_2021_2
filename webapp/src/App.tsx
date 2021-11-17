

import './App.css';
import { Routes, Route } from "react-router-dom"
import { Condominiums } from './pages/condominiums/list';
import { Home } from './pages/home';
import { QueryClient, QueryClientProvider } from "react-query"

const queryClient = new QueryClient()

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <Routes>
        <Route path="/" element={<Home/>} />
        <Route path="/condominiums" element={<Condominiums />} />
        {/* <Route path="/employees" element={<Employees />} /> */}
      </Routes>
    </QueryClientProvider>
  )
}

export default App;
