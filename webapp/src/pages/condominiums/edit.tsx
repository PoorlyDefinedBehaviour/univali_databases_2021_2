import { FormEventHandler } from "react"
import { ChangeEvent, useState } from "react"
import { useQuery } from "react-query"

import { getAll } from "../../services/cities"
import { Condominium } from "../../services/condominiums"

interface Form {
  name: string 
  cnpj: string 
  street: string 
  number: number
  cityId: number
}

interface Props {
  condominium: Condominium
}

export const EditCondominium = ({ condominium }: Props) => {
  const { isLoading, data: cities } = useQuery("get_all_cities", getAll)
  const [form, setForm] = useState<Form>({
    name: condominium.name,
    cnpj: condominium.cnpj,
    street: condominium.address.street,
    number: condominium.address.number,
    cityId: condominium.address.city.id
  })

  if(isLoading) {
    return <p>...</p>
  }

  const handleChange = (event: any) => {
    setForm(form => ({
      ...form, 
      [event.target.name]: event.target.value
    }))
  }

  const handleSubmit = (event: any) => {
    event.preventDefault()
    console.log("form",form)
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
        CNPJ
      </label>
      <input name="cnpj" value={form.cnpj} className="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500" id="grid-last-name" type="text" />
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
  </div>
  <button className="flex justify-end mt-4 bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded">
    Salvar
  </button>
  </form>
  )
}