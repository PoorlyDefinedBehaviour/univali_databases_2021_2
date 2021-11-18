import { useState } from "react"
import { useQuery } from "react-query"
import { useHistory } from "react-router-dom"
import { getAll, Condominium, deleteById }  from "../../services/condominiums"
import { EditCondominium } from "./edit"

type CondominiumProps = {
  data: Condominium
  onEdit: () => void
  onDelete: () => void
}

const Card = ({ data, onEdit, onDelete}: CondominiumProps) => (
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
      <button 
        className="flex justify-end mt-4 bg-transparent hover:bg-red-500 text-red-700 font-semibold hover:text-white py-2 px-4 border border-red-500 hover:border-transparent rounded"
        onClick={onDelete}
      >
        Excluir
      </button>
    </div>
  </div>
)
  
export const Condominiums = () => {
  const [editing, setEditing] = useState<null | Condominium>(null)
  const { isLoading, data: condominiums } = useQuery("get_all_condominiums", getAll)
  const history = useHistory()

  if(isLoading) {
    return <p>...</p>
  }

  if(editing !== null) {
    return <EditCondominium condominium={editing}/>
  }

  const handleDelete = (condominiumId: number) => {
    deleteById(condominiumId)
      .then(() => history.push("/"))
  }

  return (
   <div>
     <button 
      className="flex justify-end mt-4 bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded"
      onClick={() => history.push("/condominiums/create")}
      >
        Criar
      </button>
     { condominiums?.map(condominium =>
         <Card 
            key={condominium.id}
            data={condominium} 
            onEdit={() => setEditing(condominium)}
            onDelete={() => handleDelete(condominium.id)}
          />) 
     }
   </div>
   )
}