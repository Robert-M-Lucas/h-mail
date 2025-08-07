import React, {createContext, useContext, useState, ReactNode, useEffect} from 'react';
import {checkAuth} from "./interface.ts";


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

    useEffect(() => {
        checkAuth().then((user) => {
            if (user) setUser({name: user})
        })
    })

    if (user) {
        const logout = () => {
            setUser(null)
        }

        return <AuthContext.Provider value={{ user: user!, logout }}>
            {children}
        </AuthContext.Provider>;
    }
    else {
        return <>Log in</>
    }
};

export const useAuth = (): AuthContextType => {
    const context = useContext(AuthContext);
    if (!context) {
        throw new Error('useAuth must be used within a AuthProvider');
    }
    return context;
};
