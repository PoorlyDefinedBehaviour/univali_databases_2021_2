

import './App.css';
import { BrowserRouter, Switch, Route} from "react-router-dom"
import { Condominiums } from './pages/condominiums/list';
import { Home } from './pages/home';
import { QueryClient, QueryClientProvider } from "react-query"

const queryClient = new QueryClient()

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Switch>
          <Route exact path="/"><Home /></Route>
          <Route path="/condominiums"><Condominiums /></Route>
          {/* <Route path="/employees" element={<Employees />} /> */}
        </Switch>
      </BrowserRouter>
    </QueryClientProvider>
  )
}

export default App;
