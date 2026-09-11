const apiUrl = import.meta.env.VITE_API_URL;

if (!apiUrl) {
  throw new Error("API_URL environment variable is missing.");
}

export default {
  apiUrl,
};
