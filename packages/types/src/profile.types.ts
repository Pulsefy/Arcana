export interface IAsset3DMetadata {
  assetId: string;
  assetName: string;
  format: 'glb' | 'gltf' | 'fbx' | 'obj' | 'usd';
  sourceUrl: string;
  thumbnailUrl?: string;
  fileSizeBytes: number;
  polyCount: number;
  textureCount?: number;
  rigged: boolean;
  animations?: string[];
  uploadedAt: string;
}

export interface IArcanaProfile {
  id: string;
  userId: string;
  displayName: string;
  bio?: string;
  avatarUrl?: string;
  tags?: string[];
  model3d: IAsset3DMetadata;
  createdAt: string;
  updatedAt?: string;
}
