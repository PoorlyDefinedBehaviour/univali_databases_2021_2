import { useState } from "react"
import { useQuery } from "react-query"
import { getAll, Condominium }  from "../../services/condominiums"
import { EditCondominium } from "./edit"

type CondominiumProps = {
  data: Condominium
  onEdit: () => void
}

const Card = ({ data, onEdit }: CondominiumProps) => (
  <div className="max-w-md py-4 px-8 bg-white shadow-lg rounded-lg my-20">
    <div>
      <h2 className="text-gray-800 text-3xl font-semibold">{data.name}</h2>
      <p className="mt-2 text-gray-600">Rua {data.address.street}, {data.address.city.name}, {data.address.city.state.name}</p>
    </div>
    <div className="flex justify-end mt-4">
      <p>CNPJ {data.cnpj}</p>
    </div>
    <div className="flex justify-between">
      <button 
        className="flex justify-end mt-4 bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded"
        onClick={onEdit}
      >
        Editar
      </button>
      <button className="flex justify-end mt-4 bg-transparent hover:bg-red-500 text-red-700 font-semibold hover:text-white py-2 px-4 border border-red-500 hover:border-transparent rounded">
        Excluir
      </button>
    </div>
  </div>
)
  
export const Condominiums = () => {
  const [editing, setEditing] = useState<null | Condominium>(null)
  const { isLoading, data: condominiums } = useQuery("get_all_condominiums", getAll)

  if(isLoading) {
    return <p>...</p>
  }

  if(editing !== null) {
    return <EditCondominium condominium={editing}/>
  }


  return (
   <div>
     { condominiums?.map(condominium => <Card key={condominium.id} data={condominium} onEdit={() => setEditing(condominium)}/>) }
   </div>
   )
}