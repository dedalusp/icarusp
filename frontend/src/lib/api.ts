export async function fetchAutor(): Promise<any> {
    const response = await fetch("http://127.0.0.1:8080/autor");
    if (!response.ok) {
        throw new Error("Erro ao buscar autor");
    }
    return response.json();
}