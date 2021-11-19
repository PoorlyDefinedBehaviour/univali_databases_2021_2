

import './App.css';
import { BrowserRouter, Switch, Route} from "react-router-dom"
import { Condominiums } from './pages/condominiums/list';
import { Home } from './pages/home';
import { QueryClient, QueryClientProvider } from "react-query"
import { CreateCondominium } from './pages/condominiums/create';
import { Employees } from './pages/employees/list';
import { CreateEmployee } from './pages/employees/create';

const queryClient = new QueryClient()

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Switch>
          <Route exact path="/"><Home /></Route>
          <Route path="/condominiums/create"><CreateCondominium /></Route>
          <Route path="/condominiums"><Condominiums /></Route>
          <Route path="/employees/create"><CreateEmployee /></Route>
          <Route path="/employees"><Employees /></Route>
        </Switch>
      </BrowserRouter>
    </QueryClientProvider>
  )
}

export default App;
