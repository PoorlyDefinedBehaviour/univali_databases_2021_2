import { useQuery } from "react-query"
import { getAll, Condominium }  from "../../services/condominiums"

type CondominiumProps = {
  data: Condominium
}

const Card = ({ data }: CondominiumProps) => (
  <div className="max-w-md py-4 px-8 bg-white shadow-lg rounded-lg my-20">
    <div>
      <h2 className="text-gray-800 text-3xl font-semibold">{data.name}</h2>
      <p className="mt-2 text-gray-600">{data.address.city}</p>
    </div>
    <div className="flex justify-end mt-4">
      <p>John Doe</p>
    </div>
  </div>
)
  
export const Condominiums = () => {
  const { isLoading, data: condominiums } = useQuery("get_all_condominiums", getAll)

  if(isLoading) {
    return <p>...</p>
  }
  
  return (
    <div>
      { condominiums?.map(condominium => <Card data={condominium} />) }
    </div>
    )
}