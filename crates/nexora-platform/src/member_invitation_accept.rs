use serde::Deserialize;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemberInvitationAccept {
    pub token: String,
    pub username: String,
    pub password: String,
    pub name: String,
}

impl Drop for MemberInvitationAccept {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.token.zeroize();
        self.password.zeroize();
    }
}
