import { API_URL } from "./constants";

export interface State {
  id: number 
  name: string
}

export interface City {
  id: number 
  name: string 
  state: State
}

export const getAll = () => 
  fetch(`${API_URL}/cities`)
  .then(response => response.json())
  .then(response => (response as unknown) as City[])