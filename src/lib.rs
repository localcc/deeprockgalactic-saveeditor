pub mod deep_rock_galactic {
    use std::{array::TryFromSliceError, collections::HashMap, convert::TryInto, fs::File, io::{Write}};
    use std::error::Error;
    use serde::{Serialize, Deserialize};
    use memchr::memmem;
    
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct Cost {
        pub credits: u32,
        pub bismor: u32,
        pub croppa: u32,
        pub enor: u32,
        pub jadiz: u32,
        pub magnite: u32,
        pub umanite: u32    
    }
    
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    
    pub enum OverclockState {
        Forged,
        Unforged,
        Unacquired
    }
    
    impl Default for OverclockState {
        fn default() -> OverclockState {
            OverclockState::Unacquired
        }
    }

    
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct Overclock {
        pub class: String,
        pub weapon: String,
        pub name: String,
        pub cost: Cost,
        #[serde(skip_deserializing)]
        pub state: OverclockState
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct Cosmetic {
        pub class: String,
        pub name: String,
        pub cost: Cost,
        #[serde(skip_deserializing)]
        pub state: OverclockState
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct MatrixCores {
        pub overclocks: HashMap<String, Overclock>,
        pub cosmetics: HashMap<String, Cosmetic>
    }
    
    
    #[derive(Debug, Clone, PartialEq)]
    pub struct Minerals {
        pub bismor: f32,
        pub enor: f32,
        pub jadiz: f32,
        pub croppa: f32,
        pub magnite: f32,
        pub umanite: f32
    }
    
    impl Default for Minerals {
        fn default() -> Self {
            Minerals {
                bismor: 0f32,
                enor: 0f32,
                jadiz: 0f32,
                croppa: 0f32,
                magnite: 0f32,
                umanite: 0f32
            }
        }
    }
    
    impl Minerals {
        fn new(bismor: f32, enor: f32, jadiz: f32, croppa: f32, magnite: f32, umanite: f32) -> Minerals {
            Minerals {
                bismor, enor, jadiz, croppa, magnite, umanite
            }
        }
    }
    
    #[derive(Debug, Clone, PartialEq)]
    pub struct Brewing {
        pub yeast: f32,
        pub starch: f32,
        pub barley: f32,
        pub malt: f32
    }
    
    impl Default for Brewing {
        fn default() -> Self {
            Brewing {
                yeast: 0f32,
                starch: 0f32,
                barley: 0f32,
                malt: 0f32
            }
        }
    }
    
    impl Brewing {
        fn new(yeast: f32, starch: f32, barley: f32, malt: f32) -> Brewing {
            Brewing { yeast, starch, barley, malt }
        }
    }
        
    #[derive(Debug, Clone, PartialEq)]
    pub struct SaveFile {
        pub eng_xp: u32,
        pub scout_xp: u32,
    
        pub drill_xp: u32,
        pub gun_xp: u32,
        pub eng_num_promo: u32,
        pub scout_num_promo: u32,
        pub drill_num_promo: u32,
        pub gun_num_promo: u32,
    
        pub credits: u32,
        pub perkpoints: u32,
    
        pub blank_cores: f32,
        pub error_cores: f32,
    
        pub minerals: Minerals,
        pub brewing: Brewing,
    
        pub matrix_cores: MatrixCores,
    
        buf: Vec<u8>,
    
        eng_xp_pos: usize,
        scout_xp_pos: usize,
        drill_xp_pos: usize,
        gun_xp_pos: usize
    }

    // are those magic numbers? yes
    // do they work? yes
    const EN_MARKER: [u8; 22] = [0x85, 0xEF, 0x62, 0x6C, 0x65, 0xF1, 0x02, 0x4A, 0x8D, 0xFE, 0xB5, 0xD0, 0xF3, 0x90, 0x9D, 0x2E, 0x03, 0x00, 0x00, 0x00, 0x58, 0x50];
    const SC_MARKER: [u8; 22] = [0x30, 0xD8, 0xEA, 0x17, 0xD8, 0xFB, 0xBA, 0x4C, 0x95, 0x30, 0x6D, 0xE9, 0x65, 0x5C, 0x2F, 0x8C, 0x03, 0x00, 0x00, 0x00, 0x58, 0x50];
    const DR_MARKER: [u8; 22] = [0x9E, 0xDD, 0x56, 0xF1, 0xEE, 0xBC, 0xC5, 0x48, 0x8D, 0x5B, 0x5E, 0x5B, 0x80, 0xB6, 0x2D, 0xB4, 0x03, 0x00, 0x00, 0x00, 0x58, 0x50];
    const GU_MARKER: [u8; 22] = [0xAE, 0x56, 0xE1, 0x80, 0xFE, 0xC0, 0xC4, 0x4D, 0x96, 0xFA, 0x29, 0xC2, 0x83, 0x66, 0xB9, 0x7B, 0x03, 0x00, 0x00, 0x00, 0x58, 0x50];
    const XP_OFFSET: usize = 48;
    const NUM_PROMO_OFFSET: usize = 108;
    const CREDITS_OFFSET: usize = 33;
    const PERK_POINTS_OFFSET: usize = 36;
    const GUID_LENGTH: usize = 16;
    const MATRIX_CORES_LIST_OFFSET: usize = 141;

    //Resources
    //Minerals
    const      BISMOR: [u8; 16] = [0xAF, 0x0D, 0xC4, 0xFE, 0x83, 0x61, 0xBB, 0x48, 0xB3, 0x2C, 0x92, 0xCC, 0x97, 0xE2, 0x1D, 0xE7];
    const        ENOR: [u8; 16] = [0x48, 0x8D, 0x05, 0x14, 0x6F, 0x5F, 0x75, 0x4B, 0xA3, 0xD4, 0x61, 0x0D, 0x08, 0xC0, 0x60, 0x3E];
    const       JADIZ: [u8; 16] = [0x22, 0xBC, 0x4F, 0x7D, 0x07, 0xD1, 0x3E, 0x43, 0xBF, 0xCA, 0x81, 0xBD, 0x9C, 0x14, 0xB1, 0xAF];
    const      CROPPA: [u8; 16] = [0x8A, 0xA7, 0xFB, 0x43, 0x29, 0x3A, 0x0B, 0x49, 0xB8, 0xBE, 0x42, 0xFF, 0xE0, 0x68, 0xA4, 0x4C];
    const     MAGNITE: [u8; 16] = [0xAA, 0xDE, 0xD8, 0x76, 0x6C, 0x22, 0x7D, 0x40, 0x80, 0x32, 0xAF, 0xD1, 0x8D, 0x63, 0x56, 0x1E];
    const     UMANITE: [u8; 16] = [0x5F, 0x2B, 0xCF, 0x83, 0x47, 0x76, 0x0A, 0x42, 0xA2, 0x3B, 0x6E, 0xDC, 0x07, 0xC0, 0x94, 0x1D];

    //Brewing
    const       YEAST: [u8; 16] = [0x07, 0x85, 0x48, 0xB9, 0x32, 0x32, 0xC0, 0x40, 0x85, 0xF8, 0x92, 0xE0, 0x84, 0xA7, 0x41, 0x00];
    const      STARCH: [u8; 16] = [0x72, 0x31, 0x22, 0x04, 0xE2, 0x87, 0xBC, 0x41, 0x81, 0x55, 0x40, 0xA0, 0xCF, 0x88, 0x12, 0x80];
    const      BARLEY: [u8; 16] = [0x22, 0xDA, 0xA7, 0x57, 0xAD, 0x7A, 0x80, 0x49, 0x89, 0x1B, 0x17, 0xED, 0xCC, 0x2F, 0xE0, 0x98]; 
    const        MALT: [u8; 16] = [0x41, 0xEA, 0x55, 0x0C, 0x1D, 0x46, 0xC5, 0x4B, 0xBE, 0x2E, 0x9C, 0xA5, 0xA7, 0xAC, 0xCB, 0x06];

    // Misc?
    const ERROR_CORES: [u8; 16] = [0x58, 0x28, 0x65, 0x2C, 0x9A, 0x5D, 0xE8, 0x45, 0xA9, 0xE2, 0xE1, 0xB8, 0xB4, 0x63, 0xC5, 0x16];
    const BLANK_CORES: [u8; 16] = [0xA1, 0x0C, 0xB2, 0x85, 0x38, 0x71, 0xFB, 0x49, 0x9A, 0xC8, 0x54, 0xA1, 0xCD, 0xE2, 0x20, 0x2C];

    //Matrix cores
    const MATRIX_CORES_UNFORGED_HEADER: [u8; 66] = [0x10, 0x00, 0x00, 0x00, 0x4F, 0x77, 0x6E, 0x65, 0x64, 0x53, 0x63, 0x68, 0x65, 0x6D, 0x61, 0x74, 0x69, 0x63, 0x73, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x41, 0x72, 0x72, 0x61, 0x79, 0x50, 0x72, 0x6F, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x6D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x50, 0x72, 0x6F, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x00];
    const MATRIX_CORES_UNFORGED_FOOTER: [u8; 73] = [0x10, 0x00, 0x00, 0x00, 0x4F, 0x77, 0x6E, 0x65, 0x64, 0x53, 0x63, 0x68, 0x65, 0x6D, 0x61, 0x74, 0x69, 0x63, 0x73, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x50, 0x72, 0x6F, 0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x47, 0x75, 0x69, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];


    impl SaveFile {

        fn get_xp_offset(buf: &[u8], marker: &[u8]) -> Option<usize> {
            memmem::find_iter(buf, marker).next().and_then(|f| Some(f + XP_OFFSET))
        }

        fn get_xp(buf: &[u8], offset: usize) -> Result<u32, TryFromSliceError> {
            let eng_slice = buf[offset..offset+4].try_into()?;
            Ok(u32::from_le_bytes(eng_slice))
        }

        fn set_val(&mut self, offset: usize, val: u32) {
            let bytes = val.to_le_bytes();
            self.buf[offset] = bytes[0];
            self.buf[offset + 1] = bytes[1];
            self.buf[offset + 2] = bytes[2];
            self.buf[offset + 3] = bytes[3];
        }

        fn get_resources_pos(buf: &[u8]) -> Option<usize> {
            memmem::find_iter(buf, &String::from("OwnedResources").into_bytes()).next()
        }

        fn get_resource_val(buf: &[u8], resources_pos: usize, resource_guid: &[u8]) -> Option<f32> {
            let resource_pos = memmem::find_iter(&buf[resources_pos..buf.len()], resource_guid).next()? + GUID_LENGTH;
            Some(f32::from_le_bytes(buf[resources_pos+resource_pos..resources_pos+resource_pos+4].try_into().ok()?))
        }

        fn set_resource_val(buf: &mut [u8], resources_pos: usize, resource_guid: &[u8], val: f32) -> Option<()> {
            let resource_pos = memmem::find_iter(&buf[resources_pos..buf.len()], resource_guid).next()? + GUID_LENGTH;
            let bytes_val = val.to_le_bytes();
            let off = resources_pos+resource_pos;
            buf[off] = bytes_val[0];
            buf[off+1] = bytes_val[1];
            buf[off+2] = bytes_val[2];
            buf[off+3] = bytes_val[3];
            Some(())
        }

        fn get_brewing(buf: &[u8]) -> Option<Brewing> {
            let resources_pos = SaveFile::get_resources_pos(buf)?;
            let yeast = SaveFile::get_resource_val(buf, resources_pos, &YEAST)?;
            let starch = SaveFile::get_resource_val(buf, resources_pos, &STARCH)?;
            let barley = SaveFile::get_resource_val(buf, resources_pos, &BARLEY)?;
            let malt = SaveFile::get_resource_val(buf, resources_pos, &MALT)?;

            Some(Brewing::new(yeast, starch, barley, malt))
        }

        fn get_minerals(buf: &[u8]) -> Option<Minerals> {
            let resources_pos = SaveFile::get_resources_pos(buf)?;
            let bismor = SaveFile::get_resource_val(buf, resources_pos, &BISMOR)?;
            let enor = SaveFile::get_resource_val(buf, resources_pos, &ENOR)?;
            let jadiz = SaveFile::get_resource_val(buf, resources_pos, &JADIZ)?;
            let croppa = SaveFile::get_resource_val(buf, resources_pos, &CROPPA)?;
            let magnite = SaveFile::get_resource_val(buf, resources_pos, &MAGNITE)?;
            let umanite = SaveFile::get_resource_val(buf, resources_pos, &UMANITE)?;
            Some(Minerals::new(bismor, enor, jadiz, croppa, magnite, umanite))
        }

        fn get_credits_pos(buf: &[u8]) -> Option<usize> {
            Some(memmem::find_iter(buf, &"Credits".to_string().into_bytes()).next()? + CREDITS_OFFSET)
        }

        fn get_perkpoints_pos(buf: &[u8]) -> Option<usize> {
            memmem::find_iter(buf, &"PerkPoints".to_string().into_bytes()).next()
        }

        fn get_matrix_cores_start_pos(buf: &[u8]) -> Option<usize> {
            memmem::find_iter(buf, &"ForgedSchematics".to_string().into_bytes()).next()
        }

        fn get_matrix_cores_end_pos(buf: &[u8]) -> Option<usize> {
            memmem::find_iter(buf, &"bFirstSchematicMessageShown".to_string().into_bytes()).next()
        }

        fn load_matrix_cores(buf: &[u8], guids: &str) -> Option<MatrixCores> {
            let mut parsed_matrix_cores = serde_json::from_str::<MatrixCores>(guids).ok()?;

            let start_pos = SaveFile::get_matrix_cores_start_pos(buf)?;
            let end_pos = SaveFile::get_matrix_cores_end_pos(buf)?;

            let data_slice = &buf[start_pos..end_pos];
            
            let num_forged = u32::from_le_bytes(data_slice[63..67].try_into().ok()?); // magic numbers yay
            
            for i in 0..num_forged {
                let uuid = hex::encode(&data_slice[MATRIX_CORES_LIST_OFFSET+ (i*16) as usize ..MATRIX_CORES_LIST_OFFSET+ (i*16) as usize +16]).to_uppercase();

                if let Some(overclock) = parsed_matrix_cores.overclocks.get_mut(&uuid) {
                    overclock.state = OverclockState::Forged;
                }

                if let Some(cosmetic) = parsed_matrix_cores.cosmetics.get_mut(&uuid) {
                    cosmetic.state = OverclockState::Forged;
                }
            }

            if let Some(unforged_offset) = memmem::find_iter(data_slice, &"Owned".to_string().into_bytes()).next() {
                let unforged_count_offset = unforged_offset + 62; // and more magic numbers idk even know where they came from
                let num_unforged = u32::from_le_bytes(data_slice[unforged_count_offset..unforged_count_offset+4].try_into().ok()?);
                let unforged_pos = unforged_count_offset + 77;
                
                for i in 0..num_unforged {
                    let uuid = hex::encode(&data_slice[unforged_pos+ (i*16) as usize .. unforged_pos + (i*16) as usize + 16]).to_uppercase();

                    if let Some(overclock) = parsed_matrix_cores.overclocks.get_mut(&uuid) {
                        overclock.state = OverclockState::Unforged;
                    }

                    if let Some(cosmetic) = parsed_matrix_cores.cosmetics.get_mut(&uuid) {
                        cosmetic.state = OverclockState::Unforged;
                    }
                }
            }

            Some(parsed_matrix_cores)        
        }

        pub fn new(buf: &mut [u8], guids: &str) -> Option<Self> {
            let eng_xp_pos = SaveFile::get_xp_offset(buf, &EN_MARKER)?;
            let scout_xp_pos = SaveFile::get_xp_offset(buf, &SC_MARKER)?;
            let drill_xp_pos = SaveFile::get_xp_offset(buf, &DR_MARKER)?;
            let gun_xp_pos = SaveFile::get_xp_offset(buf, &GU_MARKER)?;

            let eng_xp = SaveFile::get_xp(buf, eng_xp_pos).ok()?;
            let scout_xp = SaveFile::get_xp(buf, scout_xp_pos).ok()?;
            let drill_xp = SaveFile::get_xp(buf, drill_xp_pos).ok()?;
            let gun_xp = SaveFile::get_xp(buf, gun_xp_pos).ok()?;

            let eng_num_promo = u32::from_le_bytes(buf[eng_xp_pos + NUM_PROMO_OFFSET..eng_xp_pos + NUM_PROMO_OFFSET + 4].try_into().ok()?);
            let scout_num_promo = u32::from_le_bytes(buf[scout_xp_pos + NUM_PROMO_OFFSET..scout_xp_pos + NUM_PROMO_OFFSET + 4].try_into().ok()?);
            let drill_num_promo = u32::from_le_bytes(buf[drill_xp_pos + NUM_PROMO_OFFSET..drill_xp_pos + NUM_PROMO_OFFSET + 4].try_into().ok()?);
            let gun_num_promo = u32::from_le_bytes(buf[gun_xp_pos + NUM_PROMO_OFFSET..gun_xp_pos + NUM_PROMO_OFFSET + 4].try_into().ok()?);
            
            let credits_pos = SaveFile::get_credits_pos(buf)?;
            let credits = u32::from_le_bytes(buf[credits_pos..credits_pos+4].try_into().ok()?);

            let perkpoints_pos = SaveFile::get_perkpoints_pos(buf);
            let perkpoints = match perkpoints_pos {
                Some(e) => u32::from_le_bytes(buf[e+PERK_POINTS_OFFSET..e+PERK_POINTS_OFFSET+4].try_into().ok()?),
                None => 0
            };

            let brewing = SaveFile::get_brewing(buf)?;
            let minerals = SaveFile::get_minerals(buf)?;

            let resources_pos = SaveFile::get_resources_pos(buf)?;
            
            let error_cores = SaveFile::get_resource_val(buf, resources_pos, &ERROR_CORES)?;
            let blank_cores = SaveFile::get_resource_val(buf, resources_pos, &BLANK_CORES)?;


            let matrix_cores = SaveFile::load_matrix_cores(buf, guids)?;

            Some(SaveFile {
                eng_xp,
                eng_num_promo,
                scout_xp,
                scout_num_promo,
                drill_xp,
                drill_num_promo,
                gun_xp,
                gun_num_promo,
                credits,
                perkpoints,
                brewing,
                minerals,
                error_cores,
                blank_cores,
                buf: buf.to_owned(),
                matrix_cores,

                eng_xp_pos,
                gun_xp_pos,
                scout_xp_pos,
                drill_xp_pos
            })
        }

        fn save_brewing(&mut self) -> Option<()> {
            let resources_pos = SaveFile::get_resources_pos(&self.buf)?;

            SaveFile::set_resource_val(&mut self.buf, resources_pos, &YEAST, self.brewing.yeast)?;
            SaveFile::set_resource_val(&mut self.buf, resources_pos, &STARCH, self.brewing.starch)?;
            SaveFile::set_resource_val(&mut self.buf, resources_pos, &BARLEY, self.brewing.barley)?;
            SaveFile::set_resource_val(&mut self.buf, resources_pos, &MALT, self.brewing.malt)?;
            Some(())
        }

        fn save_minerals(&mut self) -> Option<()> {
            let resources_pos = SaveFile::get_resources_pos(&self.buf)?;

            SaveFile::set_resource_val(&mut self.buf, resources_pos, &BISMOR, self.minerals.bismor)?;
            SaveFile::set_resource_val(&mut self.buf, resources_pos, &ENOR, self.minerals.enor)?;
            SaveFile::set_resource_val(&mut self.buf, resources_pos, &JADIZ, self.minerals.jadiz)?;
            SaveFile::set_resource_val(&mut self.buf, resources_pos, &CROPPA, self.minerals.croppa)?;
            SaveFile::set_resource_val(&mut self.buf, resources_pos, &MAGNITE, self.minerals.magnite)?;
            SaveFile::set_resource_val(&mut self.buf, resources_pos, &UMANITE, self.minerals.umanite)?;

            Some(())
        }

        fn save_unforged_matrix_cores(&mut self, buf: &mut Vec<u8>) -> Option<()> {
            let unforged_overclocks: Vec<(&String, &Overclock)> = self.matrix_cores.overclocks.iter().filter(|e| e.1.state == OverclockState::Unforged).collect();
            let unforged_cosmetics: Vec<(&String, &Cosmetic)> = self.matrix_cores.cosmetics.iter().filter(|e| e.1.state == OverclockState::Unforged).collect();
            let unforged_count = unforged_overclocks.len() as u32 + unforged_cosmetics.len() as u32;

            if unforged_count > 0 {
                buf.extend(&MATRIX_CORES_UNFORGED_HEADER);
                buf.extend(unforged_count.to_le_bytes());
                buf.extend(&MATRIX_CORES_UNFORGED_FOOTER);

                for unforged_overclock in unforged_overclocks {
                    let uuid_bytes = hex::decode(unforged_overclock.0).ok()?;
                    buf.extend(uuid_bytes);
                }

                for unforged_cosmetic in unforged_cosmetics {
                    let uuid_bytes = hex::decode(unforged_cosmetic.0).ok()?;
                    buf.extend(uuid_bytes);
                }
            }

            Some(())
        }

        fn save_matrix_cores(&mut self) -> Option<()> {
            let start_pos = SaveFile::get_matrix_cores_start_pos(&self.buf)?;
            let end_pos = SaveFile::get_matrix_cores_end_pos(&self.buf)?;

            let num_forged = u32::from_le_bytes(self.buf[start_pos+63..start_pos+67].try_into().ok()?);
            let mut new_buf = Vec::new();

            new_buf.extend(&self.buf[..start_pos + (num_forged as usize * 16) + MATRIX_CORES_LIST_OFFSET]);
            
            self.save_unforged_matrix_cores(&mut new_buf)?;

            new_buf.extend(&self.buf[end_pos..]);
            
            self.buf = new_buf;
            Some(())
        }

        pub fn save(&mut self, mut file: &File) -> Result<(), Box<dyn Error>> {
            self.set_val(self.eng_xp_pos, self.eng_xp);
            self.set_val(self.gun_xp_pos, self.gun_xp);
            self.set_val(self.scout_xp_pos, self.scout_xp);
            self.set_val(self.drill_xp_pos, self.drill_xp);

            self.set_val(SaveFile::get_perkpoints_pos(&self.buf).ok_or("Failed to get perkpoints offset")?, self.perkpoints);
            self.set_val(SaveFile::get_credits_pos(&self.buf).ok_or("Failed to get credits offset")?, self.credits);

            self.save_brewing();
            self.save_minerals();

            let resources_pos = SaveFile::get_resources_pos(&self.buf).ok_or("Failed to get resources offset")?;
            SaveFile::set_resource_val(&mut self.buf, resources_pos, &ERROR_CORES, self.error_cores);
            SaveFile::set_resource_val(&mut self.buf, resources_pos, &BLANK_CORES, self.blank_cores);

            self.save_matrix_cores().ok_or("Failed to save overclocks!")?;

            file.write_all(&self.buf)?;
            Ok(())
        }
    }
}