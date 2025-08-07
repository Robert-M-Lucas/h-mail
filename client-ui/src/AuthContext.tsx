import React, {createContext, useContext, useState, ReactNode, useEffect} from 'react';
import {checkAuth, getServer, reauthenticate, setServer} from "./interface.ts";


type AuthInfo = {
    name: string;
};

interface AuthContextType {
    user: AuthInfo;
    logout: () => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

interface AuthProviderProps {
    children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
    const [user, setUser] = useState<AuthInfo | null>(null);
    const [serverVal, setServerVal] = useState<string>("");
    const [username, setUsername] = useState<string>("");
    const [password, setPassword] = useState<string>("");
    const [error, setError] = useState<string>("");

    useEffect(() => {
        getServer().then(async (server) => {
            if (server) await setServer(server);
            checkAuth().then((user) => {
                if (user) setUser({name: user})
            })
        })
    }, [])

    if (user) {
        const logout = () => {
            setUser(null)
        }

        return <AuthContext.Provider value={{ user: user!, logout }}>
            {children}
        </AuthContext.Provider>;
    }
    else {
        const login = async () => {
            await setServer(serverVal);
            console.log("login");
            const result = await reauthenticate(username, password);
            console.log(result);
            if (result.ok) {
                setUser({name: result.value})
            }
            else {
                setError(result.error)
            }
        }

        return <>
            <h1>Log In</h1>
            <p>Server:</p>
            <input onChange={(e) => setServerVal(e.currentTarget.value)} value={serverVal}></input>
            <p>Username:</p>
            <input onChange={(e) => setUsername(e.currentTarget.value)}></input>
            <p>Password:</p>
            <input onChange={(e) => setPassword(e.currentTarget.value)}></input>
            <button onClick={() => login().then(() => {})}>Login</button>
            <p>{error}</p>
        </>
    }
};

export const useAuth = (): AuthContextType => {
    const context = useContext(AuthContext);
    if (!context) {
        throw new Error('useAuth must be used within a AuthProvider');
    }
    return context;
};
