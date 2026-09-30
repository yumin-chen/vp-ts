import {
  establishConnection as nativeEstablishConnection,
  NativeConnection,
  type Download,
  type NewDownloadParams,
  type NewPostParams,
  type NewUserParams,
  type Post,
  type User,
  type VersionModel,
} from "../index.js";

export {
  NativeConnection,
  type Download,
  type NewDownloadParams,
  type NewPostParams,
  type NewUserParams,
  type Post,
  type User,
  type VersionModel,
};

export class DieselConnection {
  private nativeConn: NativeConnection;

  constructor(databaseUrl?: string) {
    this.nativeConn = nativeEstablishConnection(databaseUrl);
    this.nativeConn.setupTables();
  }

  public get native(): NativeConnection {
    return this.nativeConn;
  }

  public execute(sql: string): number {
    return this.nativeConn.executeRaw(sql);
  }

  public loadUsers(): User[] {
    return this.nativeConn.loadUsers();
  }

  public insertUsers(usersList: NewUserParams[]): User[] {
    return this.nativeConn.insertUsers(usersList);
  }

  public updateBannedByEmail(pattern: string, banned: boolean): number {
    return this.nativeConn.updateBannedByEmail(pattern, banned);
  }

  public postsBelongingToUser(userId: number): Post[] {
    return this.nativeConn.postsBelongingToUser(userId);
  }

  public createPost(title: string, body: string, userId?: number): Post {
    return this.nativeConn.createPost(title, body, userId);
  }

  public publishPost(id: number): Post {
    return this.nativeConn.publishPost(id);
  }

  public getPost(id: number): Post | null {
    return this.nativeConn.getPost(id);
  }

  public deletePosts(target: string): number {
    return this.nativeConn.deletePosts(target);
  }

  public showPosts(limit?: number): Post[] {
    return this.nativeConn.showPosts(limit);
  }

  public insertVersion(num: string, crateId: number): VersionModel {
    return this.nativeConn.insertVersion(num, crateId);
  }

  public insertDownloads(downloads: NewDownloadParams[]): Download[] {
    return this.nativeConn.insertDownloads(downloads);
  }

  public queryDownloads(crateId: number, dateAfter: string, limit: number): Download[] {
    return this.nativeConn.queryDownloads(crateId, dateAfter, limit);
  }
}

export function establishConnection(databaseUrl?: string): DieselConnection {
  return new DieselConnection(databaseUrl);
}

// Table/Model DSL helpers matching Diesel ORM patterns
export const users = {
  table: "users",
  load(conn: DieselConnection | NativeConnection): User[] {
    const c = conn instanceof DieselConnection ? conn.native : conn;
    return c.loadUsers();
  },
  insert(conn: DieselConnection | NativeConnection, newUsersList: NewUserParams[]): User[] {
    const c = conn instanceof DieselConnection ? conn.native : conn;
    return c.insertUsers(newUsersList);
  },
};

export class PostModel {
  static belongingTo(user: User | { id: number }) {
    return {
      load(conn: DieselConnection | NativeConnection): Post[] {
        const c = conn instanceof DieselConnection ? conn.native : conn;
        return c.postsBelongingToUser(user.id);
      },
    };
  }
}

export function createPost(
  conn: DieselConnection | NativeConnection,
  title: string,
  body: string,
  userId?: number,
): Post {
  const c = conn instanceof DieselConnection ? conn.native : conn;
  return c.createPost(title, body, userId);
}

export function publishPost(conn: DieselConnection | NativeConnection, id: number): Post {
  const c = conn instanceof DieselConnection ? conn.native : conn;
  return c.publishPost(id);
}

export function getPost(conn: DieselConnection | NativeConnection, id: number): Post | null {
  const c = conn instanceof DieselConnection ? conn.native : conn;
  return c.getPost(id);
}

export function deletePost(conn: DieselConnection | NativeConnection, target: string): number {
  const c = conn instanceof DieselConnection ? conn.native : conn;
  return c.deletePosts(target);
}

export function showPosts(conn: DieselConnection | NativeConnection, limit?: number): Post[] {
  const c = conn instanceof DieselConnection ? conn.native : conn;
  return c.showPosts(limit);
}
