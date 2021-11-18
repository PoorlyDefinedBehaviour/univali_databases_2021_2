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
  number: string 
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

export const deleteById = (condominiumId: number) => 
  fetch(`${API_URL}/condominiums/${condominiumId}`, { method: "DELETE"})

export interface Create {
  name: string 
  cnpj: string 
  address: {
    street: string 
    number: string
    city_id: number
  }
}

export const create = (data: Create) => 
  fetch(`${API_URL}/condominiums`, { 
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  })
  .then(response => response.json())
  .then(response => (response as unknown) as Condominium)

export type Update = Create;

export const update = (condominiumId: number, data: Update) => 
  fetch(`${API_URL}/condominiums/${condominiumId}`, { 
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  })
  .then(response => response.json())
  .then(response => (response as unknown) as Condominium)