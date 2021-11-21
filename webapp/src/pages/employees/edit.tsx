import { useState } from "react"
import { useQuery } from "react-query"
import { useHistory } from "react-router-dom"
import { getAllShifts, getAllRoles, Employee, update } from "../../services/employees"
import { getAll as getAllCities } from "../../services/cities"
import { getAll as getAllCondominiums } from "../../services/condominiums"

interface Props {
  employee: Employee
}

interface Form {
  name: string 
  cpf: string 
  street: string 
  number: string
  cityId: number
  wageInCents: number
  condominiumId: number 
  roleId: number
  shiftId: number
}

export const EditEmployee = ({ employee }: Props) => {
  const { data: cities } = useQuery("get_all_cities", getAllCities)
  const { data: condominiums } = useQuery("get_all_condominiums", getAllCondominiums)
  const { data: shifts } = useQuery("get_all_shifts", getAllShifts)
  const { data: roles } = useQuery("get_all_roles", getAllRoles)

  const [form, setForm] = useState<Form>({
    name: employee.name,
    cpf: employee.cpf, 
    street: employee.address.street,
    number: employee.address.number,
    cityId: employee.address.city.id,
    wageInCents: employee.wage_in_cents / 100,
    condominiumId: employee.works_at_condominium.id,
    roleId: employee.role.id,
    shiftId: employee.shift.id,
  })

  const history = useHistory()

  if(!cities || !condominiums || !shifts || !roles) {
    return <p>...</p>
  }

  if(condominiums.length === 0 ){
    return <p>Crie um condomínio antes de criar um funcionário</p>
  }

  const handleChange = (event: any) => {
    setForm(form => ({
      ...form, 
      [event.target.name]: event.target.value
    }))
  }

  const handleSubmit = (event: any) => {
    event.preventDefault()
    
    update(employee.id, {
      name: form.name,
      cpf: form.cpf,
      wage_in_cents: form.wageInCents * 100,
      works_at_condominium_id: Number(form.condominiumId), 
      role_id: Number(form.roleId),
      shift_id: Number(form.shiftId),
      address: {
        city_id: Number(form.cityId),
        street: form.street, 
        number: form.number,
      }
    })
    .then((response: any) => {
      if(response.message){
        alert(response.message)
        return 
      } 

      history.push("/")
    })
  }
  
  return (
  <form className="w-full max-w-lg" onChange={handleChange} onSubmit={handleSubmit}>
  <div className="flex flex-wrap -mx-3 mb-6">
    <div className="w-full md:w-1/2 px-3 mb-6 md:mb-0">
      <label className="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" htmlFor="grid-first-name">
        Nome
      </label>
      <input name="name" value={form.name} className="appearance-none block w-full bg-gray-200 text-gray-700 border border-red-500 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white" id="grid-first-name" type="text" />
    </div>
    <div className="w-full md:w-1/2 px-3">
      <label className="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" htmlFor="grid-last-name">
        CPF
      </label>
      <input name="cpf" value={form.cpf} className="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500" id="grid-last-name" type="text" />
    </div>
    <div className="w-full md:w-1/2 px-3">
      <label className="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" htmlFor="wage-in-cents">
        Salário
      </label>
      <input name="wageInCents" value={form.wageInCents} className="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500" id="wage-in-cents" type="number" max="1000000" />
    </div>
  </div>
  <div className="flex flex-wrap -mx-3 mb-6">
    <div className="w-full px-3">
      <label className="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" htmlFor="grid-password">
        Rua
      </label>
      <input name="street" value={form.street} className="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500" id="grid-password" type="text" />
    </div>
  </div>
  <div className="flex flex-wrap -mx-3 mb-2">
    <div className="w-full md:w-1/3 px-3 mb-6 md:mb-0">
      <label className="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" htmlFor="grid-city">
        Número
      </label>
      <input name="number" value={form.number} className="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500" id="grid-city" type="text" />
    </div>
    <div className="w-full md:w-1/3 px-3 mb-6 md:mb-0">
      <label className="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" htmlFor="grid-state">
        Cidade
      </label>
      <div className="relative">
        <select name="cityId" value={form.cityId} className="block appearance-none w-full bg-gray-200 border border-gray-200 text-gray-700 py-3 px-4 pr-8 rounded leading-tight focus:outline-none focus:bg-white focus:border-gray-500" id="grid-state">
          {cities?.map(city => <option key={city.id} value={city.id}>{ city.name }</option>)}
        </select>
        <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
          <svg className="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"/></svg>
        </div>
      </div>
    </div>
    <div className="w-full md:w-1/3 px-3 mb-6 md:mb-0">
      <label className="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" htmlFor="grid-condominiums">
        Condomínio
      </label>
      <div className="relative">
        <select name="condominiumId" value={form.condominiumId} className="block appearance-none w-full bg-gray-200 border border-gray-200 text-gray-700 py-3 px-4 pr-8 rounded leading-tight focus:outline-none focus:bg-white focus:border-gray-500" id="grid-condominiums">
          {condominiums?.map(condominium => <option key={condominium.id} value={condominium.id}>{ condominium.name }</option>)}
        </select>
        <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
          <svg className="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"/></svg>
        </div>
      </div>
    </div>
    <div className="w-full md:w-1/3 px-3 mb-6 md:mb-0">
      <label className="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" htmlFor="grid-shifts">
        Turno
      </label>
      <div className="relative">
        <select name="shiftId" value={form.shiftId} className="block appearance-none w-full bg-gray-200 border border-gray-200 text-gray-700 py-3 px-4 pr-8 rounded leading-tight focus:outline-none focus:bg-white focus:border-gray-500" id="grid-shifts">
          {shifts?.map(shift => <option key={shift.id} value={shift.id}>{ shift.name }</option>)}
        </select>
        <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
          <svg className="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"/></svg>
        </div>
      </div>
    </div>
    <div className="w-full md:w-1/3 px-3 mb-6 md:mb-0">
      <label className="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" htmlFor="grid-roles">
        Cargo
      </label>
      <div className="relative">
        <select name="roleId" value={form.roleId} className="block appearance-none w-full bg-gray-200 border border-gray-200 text-gray-700 py-3 px-4 pr-8 rounded leading-tight focus:outline-none focus:bg-white focus:border-gray-500" id="grid-roles">
          {roles?.map(role => <option key={role.id} value={role.id}>{ role.name }</option>)}
        </select>
        <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
          <svg className="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"/></svg>
        </div>
      </div>
    </div>
  </div>
  <button className="flex justify-end mt-4 bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded">
    Salvar
  </button>
  </form>
  )
}