import { API_URL } from "./constants"

export interface State {
  id: number 
  name: string
}

export interface City {
  id: number
  name: string
  state: State
}

export interface Address {
  id: number 
  street: string 
  number: number 
  city: City
}

export interface Condominium {
  id: number
  name: string 
  cnpj: string 
  address: Address
}

export const getAll = () => 
  fetch(`${API_URL}/condominiums`)
  .then(response => response.json())
  .then(response => (response as unknown) as Condominium[])