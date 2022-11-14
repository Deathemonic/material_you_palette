use std::ops::Index;
use self::Role::*;
use std::slice::Iter;
use crate::palettes::core::CorePalette;

#[derive(Debug)]
pub enum Role {
    Primary,
    OnPrimary,
    PrimaryContainer,
    OnPrimaryContainer,
    Secondary,
    OnSecondary,
    SecondaryContainer,
    OnSecondaryContainer,
    Tertiary,
    OnTertiary,
    TertiaryContainer,
    OnTertiaryContainer,
    Error,
    OnError,
    ErrorContainer,
    OnErrorContainer,
    Background,
    OnBackground,
    Surface,
    OnSurface,
    SurfaceVariant,
    OnSurfaceVariant,
    Outline,
    OutlineVariant,
    Shadow,
    Scrim,
    InverseSurface,
    InverseOnSurface,
    InversePrimary,
}

// This was only needed for looping through the "scheme" to see the values
impl Role {
    pub fn iterator() -> Iter<'static, Role> {
        static ROLES: [Role; 29] = [
          Primary, OnPrimary, PrimaryContainer, OnPrimaryContainer, Secondary,
          OnSecondary, SecondaryContainer, OnSecondaryContainer, Tertiary,
          OnTertiary, TertiaryContainer, OnTertiaryContainer, Error, OnError,
          ErrorContainer, OnErrorContainer, Background, OnBackground,
          Surface, OnSurface, SurfaceVariant, OnSurfaceVariant, Outline,
          OutlineVariant, Shadow, Scrim, InverseSurface, InverseOnSurface,
          InversePrimary,];
        ROLES.iter()
    }
}

/// Represents a Material color scheme, a mapping of color roles to colors.
#[derive(Debug, Clone)]
pub struct Scheme {
    pub primary: [u8; 4],
    pub on_primary: [u8; 4],
    pub primary_container: [u8; 4],
    pub on_primary_container: [u8; 4],
    pub secondary: [u8; 4],
    pub on_secondary: [u8; 4],
    pub secondary_container: [u8; 4],
    pub on_secondary_container: [u8; 4],
    pub tertiary: [u8; 4],
    pub on_tertiary: [u8; 4],
    pub tertiary_container: [u8; 4],
    pub on_tertiary_container: [u8; 4],
    pub error: [u8; 4],
    pub on_error: [u8; 4],
    pub error_container: [u8; 4],
    pub on_error_container: [u8; 4],
    pub background: [u8; 4],
    pub on_background: [u8; 4],
    pub surface: [u8; 4],
    pub on_surface: [u8; 4],
    pub surface_variant: [u8; 4],
    pub on_surface_variant: [u8; 4],
    pub outline: [u8; 4],
    pub outline_variant: [u8; 4],
    pub shadow: [u8; 4],
    pub scrim: [u8; 4],
    pub inverse_surface: [u8; 4],
    pub inverse_on_surface: [u8; 4],
    pub inverse_primary: [u8; 4],
}

impl Index<&Role> for Scheme {
  type Output = [u8; 4];

  fn index(&self, role: &Role) -> &Self::Output {
    match &role {
      Role::Primary => &self.primary,
      Role::OnPrimary => &self.on_primary,
      Role::PrimaryContainer => &self.primary_container,
      Role::OnPrimaryContainer => &self.on_primary_container,
      Role::Secondary => &self.secondary,
      Role::OnSecondary => &self.on_secondary,
      Role::SecondaryContainer => &self.secondary_container,
      Role::OnSecondaryContainer => &self.on_secondary_container,
      Role::Tertiary => &self.tertiary,
      Role::OnTertiary => &self.on_tertiary,
      Role::TertiaryContainer => &self.tertiary_container,
      Role::OnTertiaryContainer => &self.on_tertiary_container,
      Role::Error => &self.error,
      Role::OnError => &self.on_error,
      Role::ErrorContainer => &self.error_container,
      Role::OnErrorContainer => &self.on_error_container,
      Role::Background => &self.background,
      Role::OnBackground => &self.on_background,
      Role::Surface => &self.surface,
      Role::OnSurface => &self.on_surface,
      Role::SurfaceVariant => &self.surface_variant,
      Role::OnSurfaceVariant => &self.on_surface_variant,
      Role::Outline => &self.outline,
      Role::OutlineVariant => &self.outline_variant,
      Role::Shadow => &self.shadow,
      Role::Scrim => &self.scrim,
      Role::InverseSurface => &self.inverse_surface,
      Role::InverseOnSurface => &self.inverse_on_surface,
      Role::InversePrimary => &self.inverse_primary,
    }
  }
}

impl Scheme {
    pub fn light_from_core_palette(core: &mut CorePalette) -> Scheme {
        Scheme {
            primary: core.a1.tone(40),
            on_primary: core.a1.tone(100),
            primary_container: core.a1.tone(90),
            on_primary_container: core.a1.tone(10),
            secondary: core.a2.tone(40),
            on_secondary: core.a2.tone(100),
            secondary_container: core.a2.tone(90),
            on_secondary_container: core.a2.tone(10),
            tertiary: core.a3.tone(40),
            on_tertiary: core.a3.tone(100),
            tertiary_container: core.a3.tone(90),
            on_tertiary_container: core.a3.tone(10),
            error: core.error.tone(40),
            on_error: core.error.tone(100),
            error_container: core.error.tone(90),
            on_error_container: core.error.tone(10),
            background: core.n1.tone(99),
            on_background: core.n1.tone(10),
            surface: core.n1.tone(99),
            on_surface: core.n1.tone(10),
            surface_variant: core.n2.tone(90),
            on_surface_variant: core.n2.tone(30),
            outline: core.n2.tone(50),
            outline_variant: core.n2.tone(80),
            shadow: core.n1.tone(0),
            scrim: core.n1.tone(0),
            inverse_surface: core.n1.tone(20),
            inverse_on_surface: core.n1.tone(95),
            inverse_primary: core.a1.tone(80),
        }
    }

    pub fn dark_from_core_palette(core: &mut CorePalette) -> Scheme {
        Scheme {
            primary: core.a1.tone(80),
            on_primary: core.a1.tone(20),
            primary_container: core.a1.tone(30),
            on_primary_container: core.a1.tone(90),
            secondary: core.a2.tone(80),
            on_secondary: core.a2.tone(20),
            secondary_container: core.a2.tone(30),
            on_secondary_container: core.a2.tone(90),
            tertiary: core.a3.tone(80),
            on_tertiary: core.a3.tone(20),
            tertiary_container: core.a3.tone(30),
            on_tertiary_container: core.a3.tone(90),
            error: core.error.tone(80),
            on_error: core.error.tone(20),
            error_container: core.error.tone(30),
            on_error_container: core.error.tone(90),
            background: core.n1.tone(10),
            on_background: core.n1.tone(90),
            surface: core.n1.tone(10),
            on_surface: core.n1.tone(90),
            surface_variant: core.n2.tone(30),
            on_surface_variant: core.n2.tone(80),
            outline: core.n2.tone(60),
            outline_variant: core.n2.tone(30),
            shadow: core.n1.tone(0),
            scrim: core.n1.tone(0),
            inverse_surface: core.n1.tone(90),
            inverse_on_surface: core.n1.tone(20),
            inverse_primary: core.a1.tone(40),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder_test() {
        let sum = 2 + 2;
        assert_eq!(sum, 4);
    }
}
