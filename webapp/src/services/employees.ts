import { API_URL } from "./constants";

export interface Employee {
  id: number 
  name: string 
  cpf: string 
  wage_in_cents: number 
  shift: string 
  role: string 
  address: {
    id: number 
    street: string 
    number: string 
    city: {
      id: number 
      name: string 
      state: {
        id: number 
        name: string
      }
    }
  }
}

export const getAll = () => 
  fetch(`${API_URL}/employees`)
  .then(response => response.json())
  .then(response => (response as unknown) as Employee[])

export const deleteById = (employeeId: number) => 
  fetch(`${API_URL}/employee/${employeeId}`, { method: "DELETE"})

export interface Create {
  name: string 
  cpf: String 
  wage_in_cents: number 
  works_at_condominium_id: number 
  shift_id: number 
  role_id: number 
  address: {
    street: string 
    number: string 
    city_id: number
  }
}

export const create = (data: Create) => 
  fetch(`${API_URL}/employees`, { 
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  })
  .then(response => response.json())
  .then(response => (response as unknown) as Employee)

export type Update = Create;

export const update = (employeeId: number, data: Update) => 
  fetch(`${API_URL}/employee/${employeeId}`, { 
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  })
  .then(response => response.json())
  .then(response => (response as unknown) as Employee)

export interface Shift {
  id: number 
  name: string 
}

export const getAllShifts = () => 
  fetch(`${API_URL}/employees/shifts`)
  .then(response => response.json())
  .then(response => (response as unknown) as Shift[])

export interface Role {
  id: number 
  name: string 
}
  
export const getAllRoles = () => 
  fetch(`${API_URL}/employees/roles`)
  .then(response => response.json())
  .then(response => (response as unknown) as Role[])